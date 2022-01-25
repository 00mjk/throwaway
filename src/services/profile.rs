use anyhow::Result;
use sqlx::types::Uuid;
use tracing::error;

use crate::errors::internal::ServerError;
use crate::models::database::profile::Profile;
use crate::models::request::register::RegisterRequest;
use crate::repositories::profile::ProfileRepository;
use crate::services::password::PasswordService;

#[derive(Clone)]
pub struct ProfileService {
    password_service: PasswordService,
    profile_repository: ProfileRepository,
}

impl ProfileService {
    pub const fn new(password_service: PasswordService, profile_repository: ProfileRepository) -> Self {
        Self {
            password_service,
            profile_repository,
        }
    }

    pub async fn create(&self, mut register_request: RegisterRequest) -> Result<Uuid, ServerError> {
        let profile_exists = self
            .profile_repository
            .exists(&register_request.email)
            .await?;

        if profile_exists {
            error!("Profile already exists");
            return Err(ServerError::GenericError);
        }

        let password_hash = self
            .password_service
            .hash(&register_request.password.clone())
            .await?;

        register_request.password = password_hash;

        let profile_id = self
            .profile_repository
            .insert(register_request)
            .await?;

        Ok(profile_id)
    }

    pub async fn read(&self, profile_id: Uuid) -> Result<Profile, ServerError> {
        let profile = self
            .profile_repository
            .get_by_profile_id(profile_id)
            .await?;

        Ok(profile)
    }

    pub async fn update(&self, profile: Profile) -> Result<(), ServerError> {
        self.profile_repository
            .update(profile)
            .await?;

        Ok(())
    }

    pub async fn update_name(&self, profile_id: Uuid, name: String) -> Result<Profile, ServerError> {
        let mut profile: Profile = self.read(profile_id).await?;
        if name == profile.name {
            error!("Name already set");
            return Err(ServerError::GenericError);
        }

        profile.name = name.clone();
        self.profile_repository
            .update_name(profile_id, name)
            .await?;

        Ok(profile)
    }

    pub async fn update_email(&self, profile_id: Uuid, email: String) -> Result<Profile, ServerError> {
        let mut profile: Profile = self.read(profile_id).await?;
        if email == profile.email {
            error!("Email already set");
            return Err(ServerError::GenericError);
        }

        profile.name = email.clone();
        self.profile_repository
            .update_email(profile_id, email)
            .await?;

        Ok(profile)
    }

    // pub async fn delete(&self, profile_id: Uuid) -> Result<Profile, ServerError> {
    //     todo!()
    // }

    pub async fn verify_credentials(&self, email: String, password: String) -> Result<(bool, Profile), ServerError> {
        let profile = self
            .profile_repository
            .get_by_email(&email)
            .await?;

        let valid = self
            .password_service
            .verify(&password, &profile.password)
            .await;

        Ok((valid, profile))
    }
}
