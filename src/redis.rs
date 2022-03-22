#![warn(clippy::all)]

use std::time::Duration;

use redis::{AsyncCommands, Client, RedisError as Error};

use crate::SessionManager;

#[async_trait]
impl SessionManager for Client {
    type Error = Error;
    async fn set(&self, id: &str, key: &str, value: &str, time: Duration) -> Result<(), Error> {
        let key = format!("{key}:{id}");
        let secs = time
            .as_secs()
            .try_into()
            .unwrap_or(usize::MAX);
        #[cfg(feature = "log")]
        log::debug!("set \"{key}\" to \"{value}\" expiring in {secs} seconds.");
        self.get_async_connection()
            .await?
            .set_ex(key, &*value, secs)
            .await?;
        Ok(())
    }

    async fn delete(&self, id: &str, key: &str) -> Result<(), Error> {
        #[cfg(feature = "log")]
        log::debug!("delete \"{key}:{id}\"");
        self.get_async_connection()
            .await?
            .del(format!("{key}:{id}"))
            .await?;

        Ok(())
    }

    async fn get(&self, id: &str, key: &str) -> Result<Option<String>, Error> {
        #[cfg(feature = "log")]
        log::debug!("get {key}");
        let option: Option<String> = self
            .get_async_connection()
            .await?
            .get(format!("{key}:{id}"))
            .await?;
        Ok(option)
    }

    async fn expire_in(&self, id: &str, key: &str, time: Duration) -> Result<(), Error> {
        let key = format!("{key}:{id}");
        let secs = time
            .as_secs()
            .try_into()
            .unwrap_or(usize::MAX);
        #[cfg(feature = "log")]
        log::debug!("set expiration for \"{key}\" in {secs} seconds.");
        self.get_async_connection()
            .await?
            .expire(key, secs)
            .await?;
        Ok(())
    }

    async fn clear_all(&self) -> Result<(), Error> {
        #[cfg(feature = "log")]
        log::warn!("flushing all redis keys.");
        let mut cnn = self
            .get_async_connection()
            .await?;
        redis::Cmd::new()
            .arg("FLUSHDB")
            .query_async::<_, ()>(&mut cnn)
            .await?;
        Ok(())
    }
}
