use serde::Serialize;

use crate::Config;

#[derive(Serialize, Debug)]
pub struct ConfigResponse {
    pub config: Config,
}
