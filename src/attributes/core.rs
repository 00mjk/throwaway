use serde::{Deserialize, Serialize};

use crate::attributes::profile::ProfileAttributes;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Attributes {
    profile: Option<ProfileAttributes>,
}
