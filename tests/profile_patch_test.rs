use anyhow::Error;
use serde_json::json;

mod common;

use common::models::profile::profile_create::CreateProfile;
use common::models::profile::profile_patch::PatchProfile;

use crate::common::framework::Framework;

#[tokio::test]
async fn profile_patch_name_valid() -> Result<(), Error> {
    let framework = Framework::new().await;

    // Register
    let profile = CreateProfile::new();
    framework
        .create_profile(&profile)
        .await?;

    // Fetch Token
    let token_request_body = json!({
        "lifespan": 60,
        "attributes": {
            "profile": {
                "update": true
            }
        }
    });

    let token_response = framework
        .request_token(&profile.email, &profile.password, token_request_body)
        .await?;

    let token_authorization = token_response.body["token"]
        .as_str()
        .unwrap();

    // Patch Profile
    let profile_patch = PatchProfile {
        name: Some("Updated".to_string()),
        email: None,
        country: None,
        timezone: None,
    };

    let patch_response = framework
        .patch_profile(token_authorization, &profile_patch)
        .await?;

    let patch_response_country = patch_response.body["profile"]["name"]
        .as_str()
        .unwrap();

    assert_eq!(patch_response_country, "Updated");

    Ok(())
}
