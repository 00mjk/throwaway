use chrono::{DateTime, Utc};
use redis::{FromRedisValue, RedisResult, RedisWrite, ToRedisArgs, Value};
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Profile {
    pub profile_id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub country: String,
    pub timezone: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl ToRedisArgs for &Profile {
    fn write_redis_args<W>(&self, output: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        output.write_arg_fmt(serde_json::to_string(self).unwrap());
    }
}

impl FromRedisValue for Profile {
    fn from_redis_value(value: &Value) -> RedisResult<Self> {
        match *value {
            redis::Value::Data(ref value_slice) => {
                match serde_json::from_slice(value_slice) {
                    Err(_) => Err((redis::ErrorKind::TypeError, "Can't serialize value").into()),
                    Ok(profile) => Ok(profile),
                }
            }
            _ => Err((redis::ErrorKind::ResponseError, "Response type not Profile compatible.").into()),
        }
    }
}
