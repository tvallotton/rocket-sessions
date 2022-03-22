use std::time::Duration;

use redis::Client;
use rocket_sessions::{in_memory::InMemory, SessionManager};

fn random() -> String {
    use rand::{distributions::Alphanumeric, Rng};
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

async fn set_and_get<S: SessionManager>(client: S) {
    let ref id = random();
    let key = "crsf_token";
    let ref value = random();
    client
        .set(id, key, value, Duration::from_secs(100))
        .await
        .unwrap();
    let new_value = client
        .get(id, key)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(value, &new_value);

    client
        .delete(id, key)
        .await
        .unwrap();

    let new_value = client
        .get(id, key)
        .await
        .unwrap();
    assert_eq!(new_value, None);
}

async fn expiration<S: SessionManager>(client: S) {
    let ref id = random();
    let key = "crsf_token";
    let ref value = random();
    client
        .set(id, key, value, Duration::from_secs(100))
        .await
        .unwrap();

    client
        .expire_in(id, key, Duration::from_secs(1))
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(1)).await;

    let option = client
        .get(id, key)
        .await
        .unwrap();
    assert_eq!(option, None);
}

#[tokio::test]
async fn redis() {
    env_logger::try_init().ok();
    let client = Client::open("redis://127.0.0.1/").unwrap();
    tokio::join!(set_and_get(client.clone()), expiration(client));
}

#[tokio::test]
async fn in_memory() {
    env_logger::try_init().ok();
    let in_mem = InMemory::default();
    tokio::join!(set_and_get(in_mem.clone()), expiration(in_mem));
}
