use kanaeru::{rabbitmq::AmqpPool, redis::RedisConnection};

pub struct SessionService {
    pub config_store: RedisConnection,
    pub mq: AmqpPool,
    pub redis: RedisConnection,
}
