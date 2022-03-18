use sqlx::types::Uuid;
use tracing::error;
use tracing::instrument;

use crate::database::DatabasePool;
use crate::errors::core::ServerError;
use crate::errors::profile::ProfileError;
use crate::models::database::profile::Profile;
use crate::models::request::register::RegisterRequest;
use crate::Cache;

#[derive(Clone)]
pub struct ProfileRepository {
    pub database: DatabasePool,
    pub cache: Cache,
}

// FIXME: I wonder if splitting this into SQL v Cache repos (?) would be nicer?
impl ProfileRepository {
    #[must_use]
    pub const fn new(database: DatabasePool, cache: Cache) -> Self {
        Self {
            database,
            cache,
        }
    }

    // FIXME: Probably shouldn't pass in request object directly, pass in parts instead.
    #[instrument(skip(self), level = "info")]
    pub async fn insert(&self, register_request: RegisterRequest) -> Result<Uuid, ServerError> {
        // language=sql
        let query = sqlx::query!(
            r#"
            INSERT INTO throwaway.profile (name, email, password, country, timezone)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING (profile_id)
            "#,
            register_request.name,
            register_request.email,
            register_request.password,
            register_request.country,
            register_request.timezone,
        );

        return match query.fetch_one(&self.database).await {
            Ok(output) => Ok(output.profile_id),
            Err(error) => {
                // Tracing instead? Then we don't need to log out parameters every time
                error!("Could not insert Profile: {error:#?}");
                Err(ProfileError::NotFound.into())
            }
        };
    }

    #[instrument(skip(self), level = "info")]
    pub async fn exists(&self, email: &str) -> Result<bool, ServerError> {
        if self.cache_exists_by_email(email).await {
            return Ok(true);
        }

        // language=sql
        let query = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM throwaway.profile
                WHERE email = $1
            )
            "#,
            email,
        );

        return match query.fetch_one(&self.database).await {
            Ok(output) => {
                let exists: bool = output.exists.unwrap();
                Ok(exists)
            }
            Err(error) => {
                error!("Could not find existing Profile by email: {error:#?}");
                Err(ProfileError::NotFound.into())
            }
        };
    }

    #[instrument(skip(self), level = "info")]
    pub async fn get_by_email(&self, email: &str) -> Result<Profile, ServerError> {
        if let Some(profile) = self.cache_get_by_email(email).await {
            return Ok(profile);
        }

        // language=sql
        let query = sqlx::query_as!(
            Profile,
            r#"
            SELECT profile_id, name, email, password, country, timezone, created_at, updated_at, is_deleted
            FROM throwaway.profile
            WHERE email = $1
            "#,
            email,
        );

        return match query.fetch_one(&self.database).await {
            Ok(profile) => {
                self.cache_set(profile.clone()).await;
                Ok(profile)
            }
            Err(error) => {
                error!("Could not find Profile by email: {error:#?}");
                Err(ProfileError::NotFound.into())
            }
        };
    }

    #[instrument(skip(self), level = "info")]
    pub async fn get_by_profile_id(&self, profile_id: Uuid) -> Result<Profile, ServerError> {
        if let Some(profile) = self.cache_get_by_id(profile_id).await {
            return Ok(profile);
        }

        // language=sql
        let query = sqlx::query_as!(
            Profile,
            r#"
            SELECT profile_id, name, email, password, country, timezone, created_at, updated_at, is_deleted
            FROM throwaway.profile
            WHERE profile_id = $1
            "#,
            profile_id,
        );

        return match query.fetch_one(&self.database).await {
            Ok(profile) => {
                self.cache_set(profile.clone()).await;
                Ok(profile)
            }
            Err(error) => {
                error!("Could not find Profile by ID: {error:#?}");
                Err(ProfileError::NotFound.into())
            }
        };
    }

    #[instrument(skip(self), level = "info")]
    pub async fn update(&self, profile: Profile) -> Result<(), ServerError> {
        // language=sql
        let query = sqlx::query!(
            r#"
            UPDATE throwaway.profile
            SET
                name = $1,
                email = $2,
                password = $3,
                country = $4,
                timezone = $5
            WHERE profile_id = $6
            "#,
            profile.name,
            profile.email,
            profile.password,
            profile.country,
            profile.timezone,
            profile.profile_id,
        );

        return match query.execute(&self.database).await {
            Ok(_) => Ok(()),
            Err(error) => {
                // FIXME: Do we log out here or at the "error" mapping level?
                error!("Failed to update Profile: {error:#?}");
                Err(ProfileError::UpdateFailure.into())
            }
        };
    }

    #[instrument(skip(self), level = "info")]
    pub async fn update_name(&self, profile_id: Uuid, name: String) -> Result<(), ServerError> {
        // language=sql
        let query = sqlx::query!(
            r#"
            UPDATE throwaway.profile
            SET name = $1
            WHERE profile_id = $2
            "#,
            name,
            profile_id,
        );

        return match query.execute(&self.database).await {
            Ok(_) => Ok(()),
            Err(error) => {
                error!("Failed to update Profile name: {error:#?}");
                Err(ProfileError::UpdateFailure.into())
            }
        };
    }

    #[instrument(skip(self), level = "info")]
    pub async fn update_email(&self, profile_id: Uuid, email: String) -> Result<(), ServerError> {
        // language=sql
        let query = sqlx::query!(
            r#"
            UPDATE throwaway.profile
            SET email = $1
            WHERE profile_id = $2
            "#,
            email,
            profile_id,
        );

        return match query.execute(&self.database).await {
            Ok(_) => Ok(()),
            Err(error) => {
                error!("Failed to update Profile email: {error:#?}");
                Err(ProfileError::UpdateFailure.into())
            }
        };
    }

    // NOTE: Seems like we shouldn't invalidate caches at all, simply update? Might no scale quite as well...
    pub async fn cache_set(&self, profile: Profile) {
        let cache_key_email = Self::cache_key_email(&profile.email);
        self.cache
            .set(&cache_key_email, &profile, 600)
            .await;

        let cache_key_id = Self::cache_key_id(profile.profile_id);
        self.cache
            .set(&cache_key_id, &profile, 600)
            .await;
    }

    pub async fn cache_get_by_id(&self, profile_id: Uuid) -> Option<Profile> {
        let cache_key = Self::cache_key_id(profile_id);
        self.cache.get(&cache_key).await
    }

    // FIXME: Why do we need email again? For JWT lookups, why not just store Uuid in JWT and call it a day?
    // FIXME: The "best practices" i followed seemed to stick with email as the claim in use for identification.
    pub async fn cache_get_by_email(&self, email: &str) -> Option<Profile> {
        let cache_key = Self::cache_key_email(email);
        self.cache.get(&cache_key).await
    }

    pub async fn cache_exists_by_email(&self, email: &str) -> bool {
        let cache_key = Self::cache_key_email(email);
        self.cache.exists(&cache_key).await
    }

    pub fn cache_key_id(profile_id: Uuid) -> String {
        format!("profile_id_{profile_id}")
    }

    pub fn cache_key_email(email: &str) -> String {
        format!("profile_email_{email}")
    }
}
