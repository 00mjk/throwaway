use sqlx::types::Uuid;

use crate::core::errors::ServerError;
use crate::database::DatabasePool;
use crate::models::database::profile::Profile;
use crate::models::request::register::RegisterRequest;
use crate::Cache;

#[derive(Clone)]
pub struct ProfileRepository {
    pub database: DatabasePool,
    pub cache: Cache,
}

impl ProfileRepository {
    pub const fn new(database: DatabasePool, cache: Cache) -> Self {
        Self {
            database,
            cache,
        }
    }

    pub async fn insert(&self, register_request: RegisterRequest) -> Uuid {
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

        let profile_id: Uuid = query
            .fetch_one(&self.database)
            .await
            .unwrap()
            .profile_id;

        profile_id
    }

    pub async fn exists(&self, email: &str) -> Result<bool, ServerError> {
        let cache_key = format!("profile_exists_{}", email);
        if let Some(cache_result) = self.cache.get(&cache_key).await {
            return Ok(cache_result);
        }

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
                if exists {
                    self.cache
                        .set(&cache_key, exists, 600)
                        .await;
                }

                Ok(exists)
            }
            Err(_) => Err(ServerError::Internal("".to_string())),
        };
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Profile, ServerError> {
        let cache_key = format!("profile_email_{}", email);
        if let Some(profile) = self.cache.get(&cache_key).await {
            return Ok(profile);
        }

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
                self.cache
                    .set(&cache_key, &profile, 600)
                    .await;

                Ok(profile)
            }
            Err(_) => Err(ServerError::Internal("".to_string())),
        };
    }

    pub async fn get_by_profile_id(&self, profile_id: Uuid) -> Result<Profile, ServerError> {
        let cache_key = format!("profile_id_{}", profile_id);
        if let Some(profile) = self.cache.get(&cache_key).await {
            return Ok(profile);
        }

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
                self.cache
                    .set(&cache_key, &profile, 600)
                    .await;

                Ok(profile)
            }
            Err(_) => Err(ServerError::Internal("".to_string())),
        };
    }

    pub async fn update(&self, profile: Profile) -> Result<bool, ServerError> {
        let query = sqlx::query_as!(
            Profile,
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

        return match query.fetch_one(&self.database).await {
            Ok(_) => Ok(true),
            Err(_) => Err(ServerError::Internal("".to_string())),
        };
    }
}
