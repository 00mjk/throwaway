use serde::{Deserialize, Serialize};

use crate::helpers::serialization::is_none_or_false;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProfileAttributes {
    #[serde(skip_serializing_if = "is_none_or_false")]
    pub create: Option<bool>,

    #[serde(skip_serializing_if = "is_none_or_false")]
    pub read: Option<bool>,

    #[serde(skip_serializing_if = "is_none_or_false")]
    pub update: Option<bool>,

    #[serde(skip_serializing_if = "is_none_or_false")]
    pub delete: Option<bool>,
}
