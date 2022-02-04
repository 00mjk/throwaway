use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatabaseSecrets {
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub db: String,
}

impl DatabaseSecrets {
    pub fn dsn(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db,
        )
    }

    pub fn local_dsn(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.password, "localhost", self.port, self.db,
        )
    }
}
