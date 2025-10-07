use kanaeru::{rabbitmq::AmqpPool, redis::RedisConnection, sqlx::DatabaseProcessor};

use crate::{services::session::SessionService, utils::argon2::Argon2PasswordAlgorithm};

pub struct EmailAccountService {
    pub db: DatabaseProcessor,
    pub config_store: RedisConnection,
    pub mq: AmqpPool,
    pub session_service: SessionService,
    pub password_algorithm: Argon2PasswordAlgorithm<'static>,
}
