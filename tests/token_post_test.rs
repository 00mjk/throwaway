use anyhow::Error;
use serde_json::json;

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
    // FIXME: Maybe use a builder here to create a token request body instead, shouldn't necessarily be testing by
    // passing in raw JSON bodies, try and make this more approachable?
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
    assert!(!token_authorization.is_empty());

    Ok(())
}
