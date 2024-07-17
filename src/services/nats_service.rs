use nats::asynk::Connection;
use nats::Options;
use nats::Error;
use tokio::task;

pub async fn connect_to_nats(nats_url: &str) -> Result<Connection, Error> {
    task::spawn_blocking(move || Options::new().connect(nats_url))
        .await
        .map_err(|e| Error::new(e.to_string()))?
}

pub async fn publish_to_nats(nats: &Connection, subject: &str, message: &[u8]) -> Result<(), Error> {
    nats.publish(subject, message).await
}
