use nats::asynk::Connection;
use nats::Options;
use nats::Error;

pub async fn connect_to_nats(nats_url: &str) -> Result<Connection, Error> {
    Options::with_user_pass(&nats_url, "", "").connect_async().await
}

pub async fn publish_to_nats(nats: &Connection, subject: &str, message: &[u8]) -> Result<(), Error> {
    nats.publish(subject, message).await
}