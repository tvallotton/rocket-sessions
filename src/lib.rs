#![warn(clippy::all)]
#[macro_use]
extern crate async_trait;

use std::fmt::{Debug, Display};
use std::time::Duration;

#[cfg(feature = "redis")]
mod redis;

#[cfg(feature = "in-memory")]
pub mod in_memory;

/// Represents a handle to the underlying data structure
/// managing sessions in a Rocket application.
/// The `id` parameter corresponds to the session token
/// stored by the client. This should correspond to a
/// random alphanumeric string of letters with sufficient entropy.
///
/// The `key` and `value` parameters can be used to set custom fields. They could
/// be used to store CRSF tokens, timestamps or some other session metadata.
#[async_trait]
pub trait SessionManager {
    type Error: Debug + Display;
    async fn set(
        &self,
        id: &str,
        key: &str,
        value: &str,
        time: Duration,
    ) -> Result<(), Self::Error>;
    async fn get(&self, id: &str, key: &str) -> Result<Option<String>, Self::Error>;
    async fn delete(&self, id: &str, key: &str) -> Result<(), Self::Error>;
    async fn expire_in(&self, id: &str, key: &str, time: Duration) -> Result<(), Self::Error>;
    async fn clear_all(&self) -> Result<(), Self::Error>;
}
