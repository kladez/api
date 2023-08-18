use rdkafka::{
    consumer::{
        Consumer as _,
        StreamConsumer,
    },
    message::ToBytes,
    producer::{
        FutureProducer,
        FutureRecord,
    },
    ClientConfig,
};

use crate::Config;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("kafka error: {0}")]
    Kafka(#[from] rdkafka::error::KafkaError),
    #[error("futures channel canceled error: {0}")]
    FuturesChannelCanceled(#[from] futures_channel::oneshot::Canceled),
}

#[derive(Debug)]
pub enum Topic {
    UserRegistrations,
}

impl std::fmt::Display for Topic {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        match self {
            Self::UserRegistrations => write!(f, "user_registrations"),
        }
    }
}

#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Kafka {
    brokers: String,
    #[derivative(Debug = "ignore")]
    producer: FutureProducer,
}

impl Kafka {
    pub fn new(config: &Config) -> Self {
        let brokers = config.kafka_brokers.clone();

        let producer = ClientConfig::new()
            .set("bootstrap.servers", &brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("kafka producer creation error");

        Self { brokers, producer }
    }

    pub async fn send<'a, K, P>(
        &self,
        topic: Topic,
        key: &'a K,
        payload: &'a P,
    ) -> Result<(i32, i64), Error>
    where
        K: ToBytes + ?Sized,
        P: ToBytes + ?Sized,
    {
        let topic = topic.to_string();
        let record = FutureRecord::to(&topic).key(key).payload(payload);

        self.producer
            .send_result(record)
            .map_err(|e| e.0)?
            .await?
            .map_err(|e| e.0)?;

        Ok((0, 0))
    }

    pub fn get_consumer(
        &self,
        topics: &[Topic],
    ) -> Result<Consumer, Error> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &self.brokers)
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            .set("group.id", "kladez")
            .create()?;

        let topics = topics.iter().map(|t| t.to_string()).collect::<Vec<_>>();
        let topics = topics.iter().map(AsRef::as_ref).collect::<Vec<_>>();

        consumer.subscribe(&topics)?;

        let consumer = Consumer { consumer };

        Ok(consumer)
    }
}

#[allow(missing_debug_implementations)]
pub struct Consumer {
    pub consumer: StreamConsumer,
}

impl Consumer {
    pub async fn recv(&self) -> Result<rdkafka::message::BorrowedMessage, Error> {
        let message = self.consumer.recv().await?;
        Ok(message)
    }
}
