use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CacheSecrets {
    pub host: String,
    pub port: String,
    pub password: String,
}

impl CacheSecrets {
    pub fn dsn(&self) -> String {
        format!("redis://:{}@{}:{}", self.password, self.host, self.port)
    }

    pub fn local_dsn(&self) -> String {
        format!("redis://:{}@{}:{}", self.password, "localhost", self.port)
    }
}
