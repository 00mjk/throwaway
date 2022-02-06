use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::attributes::core::Attributes;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct TokenRequest {
    /// FIXME: Limit options here? (10mins, 1hr, 6hrs, 12hrs, 24hrs etc. not the most user friendly...)
    /// Maybe just 'X seconds' is good enough, but we'll need to limit max here (use validate)
    pub lifespan: usize,

    /// FIXME: How can we validate this? Will be tricky.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Attributes>,
}
