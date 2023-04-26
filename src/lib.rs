pub use bb8;

use async_trait::async_trait;
use redis::cluster::ClusterClient;
use redis::cluster_async::ClusterConnection;
use redis::{ErrorKind, IntoConnectionInfo, RedisError};

/// A `bb8::ManageConnection` for `redis_cluster_async::Client::get_connection`.
#[derive(Clone)]
pub struct RedisConnectionManager {
    client: ClusterClient,
}

// Because redis_cluster_async::Client does not support Debug derive.
impl std::fmt::Debug for RedisConnectionManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisConnectionManager")
            .field("client", &format!("pointer({:p})", &self.client))
            .finish()
    }
}

impl RedisConnectionManager {
    /// Create a new `RedisConnectionManager`.
    /// See `redis_cluster_async::Client::open` for a description of the parameter types.
    pub fn new<T: IntoConnectionInfo>(infos: Vec<T>) -> Result<RedisConnectionManager, RedisError> {
        Ok(RedisConnectionManager {
            client: ClusterClient::new(infos)?,
        })
    }
}

#[async_trait]
impl bb8::ManageConnection for RedisConnectionManager {
    type Connection = ClusterConnection;
    type Error = RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.client.get_async_connection().await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let pong: String = redis::cmd("PING").query_async(conn).await?;
        match pong.as_str() {
            "PONG" => Ok(()),
            _ => Err((ErrorKind::ResponseError, "ping request").into()),
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
