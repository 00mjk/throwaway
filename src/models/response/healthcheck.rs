use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct HealthcheckResponse {
    pub ok: bool,
    pub database: HealthDatabaseResponse,
}

#[derive(Serialize, Debug)]
pub struct HealthDatabaseResponse {
    pub tables: HashMap<String, bool>,
}
