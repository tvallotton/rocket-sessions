use std::convert::Infallible;
use std::time::{Duration, Instant};

use chashmap::CHashMap;

use crate::SessionManager;

#[derive(Default, Clone)]
pub struct InMemory(CHashMap<(String, String), (Instant, String)>);

fn future_instant(dur: Duration) -> Instant {
    Instant::now()
        .checked_add(dur)
        .expect("could not add `Duration` to `Instant`")
}

fn expired(instant: &Instant) -> bool {
    instant
        .checked_duration_since(Instant::now())
        .is_none()
}

#[async_trait]
impl SessionManager for InMemory {
    type Error = Infallible;
    async fn set(&self, id: &str, key: &str, val: &str, dur: Duration) -> Result<(), Self::Error> {
        let id = id.into();
        let key = key.into();
        let val = val.into();
        let dur = dur.into();
        let instant = future_instant(dur);
        #[cfg(feature = "log")]
        log::debug!(
            "set \"{key}\" to \"{val}\" expiring in {} seconds.",
            dur.as_secs()
        );
        self.0
            .insert((id, key), (instant, val));
        Ok(())
    }
    async fn get(&self, id: &str, key: &str) -> Result<Option<String>, Self::Error> {
        #[cfg(feature = "log")]
        log::debug!("get {key}");
        let key = (id.into(), key.into());
        let guard = self.0.get(&key);
        let (time, val) = if let Some(value) = guard.as_deref() {
            value
        } else {
            return Ok(None);
        };

        if expired(time) {
            // required before delete to avoid deadlocks
            drop(guard);
            self.delete(&key.0, &key.1)
                .await?;
            Ok(None)
        } else {
            Ok(Some(val.clone()))
        }
    }
    async fn delete(&self, id: &str, key: &str) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::debug!("delete \"{key}:{id}\"");
        let key = (id.into(), key.into());
        self.0.remove(&key);
        Ok(())
    }
    async fn expire_in(&self, id: &str, key: &str, time: Duration) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::debug!(
            "set expiration for \"{key}\" in {} seconds.",
            time.as_secs()
        );
        let option = self
            .0
            .get_mut(&(id.into(), key.into()));
        if let Some(mut value) = option {
            value.0 = future_instant(time);
        }

        Ok(())
    }
    async fn clear_all(&self) -> Result<(), Self::Error> {
        #[cfg(feature = "log")]
        log::warn!("clearing all keys.");
        self.0.clear();
        Ok(())
    }
}
