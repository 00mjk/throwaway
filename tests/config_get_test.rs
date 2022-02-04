use anyhow::Error;
mod common;

use common::models::profile::profile_create::CreateProfile;
use common::models::profile::profile_patch::PatchProfile;

use crate::common::framework::Framework;

#[tokio::test]
async fn config_get_valid() -> Result<(), Error> {
    let framework = Framework::new().await;

    // Lookup config
    let config_response = framework.get_config().await?;
    let use_local_string = config_response.body["config"]["use_local"]
        .as_bool()
        .unwrap();

    assert_eq!(use_local_string, true);

    Ok(())
}
