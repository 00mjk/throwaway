use anyhow::Error;
mod common;

use common::models::profile::profile_create::CreateProfile;
use common::models::profile::profile_patch::PatchProfile;

use crate::common::framework::Framework;

#[tokio::test]
async fn version_get_valid() -> Result<(), Error> {
    let framework = Framework::new().await;

    // Lookup version
    let version_response = framework.get_version().await?;

    let version_string = version_response.body["version"]
        .as_str()
        .unwrap();
    assert!(!version_string.is_empty());

    Ok(())
}
