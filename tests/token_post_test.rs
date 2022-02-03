use anyhow::Error;
mod common;

use common::models::profile::profile_create::CreateProfile;
use common::models::profile::profile_patch::PatchProfile;

use crate::common::framework::Framework;

#[tokio::test]
async fn token_post_valid() -> Result<(), Error> {
    let framework = Framework::new().await;

    // Register
    let profile = CreateProfile::new();
    framework
        .create_profile(&profile)
        .await?;

    // Fetch Token
    let token_response = framework
        .request_token(&profile.email, &profile.password)
        .await?;

    let token_authorization = token_response.body["token"]
        .as_str()
        .unwrap();

    assert!(!token_authorization.is_empty());

    Ok(())
}
