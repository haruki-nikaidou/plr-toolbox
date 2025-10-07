use argon2::password_hash::Error as Argon2Error;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};

#[derive(Debug, thiserror::Error)]
pub enum PasswordHashError {
    #[error("io error: {0}")]
    Io(std::io::Error),
    #[error("invalid config")]
    InvalidConfig,
    #[error("encoding failed")]
    EncodingFailed,
    #[error("decode failed")]
    DecodeFailed,
    #[error("unknown error")]
    UnknownError,
}

impl From<Argon2Error> for PasswordHashError {
    fn from(value: Argon2Error) -> Self {
        match value {
            Argon2Error::Algorithm
            | Argon2Error::OutputSize { .. }
            | Argon2Error::ParamNameDuplicated
            | Argon2Error::ParamNameInvalid
            | Argon2Error::ParamValueInvalid(_)
            | Argon2Error::ParamsMaxExceeded
            | Argon2Error::Version => PasswordHashError::InvalidConfig,
            Argon2Error::B64Encoding(_) | Argon2Error::Crypto | Argon2Error::SaltInvalid(_) => {
                PasswordHashError::EncodingFailed
            }
            Argon2Error::Password => PasswordHashError::DecodeFailed,
            _ => PasswordHashError::UnknownError,
        }
    }
}

/// Password hashing and verification
pub trait PasswordAlgorithm {
    /// Hashes a password
    fn hash_password(&self, password: &str) -> Result<String, PasswordHashError>;

    /// Verifies a password
    fn verify_password(&self, password: &str, hash: &str) -> bool;
}

/// Argon2 password hashing and verification
#[derive(Debug, Clone, Default)]
pub struct Argon2PasswordAlgorithm<'a> {
    config: argon2::Argon2<'a>,
}

impl<'a> Argon2PasswordAlgorithm<'a> {
    /// Creates a new [Argon2PasswordAlgorithm] with the given config
    pub fn new(config: argon2::Argon2<'a>) -> Self {
        Self { config }
    }
}

impl<'a> PasswordAlgorithm for Argon2PasswordAlgorithm<'a> {
    fn hash_password(&self, password: &str) -> Result<String, PasswordHashError> {
        let salt = SaltString::generate(&mut OsRng);
        match self.config.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => Ok(hash.to_string()),
            Err(e) => Err(e.into()),
        }
    }

    fn verify_password(&self, password: &str, hash: &str) -> bool {
        let Ok(parsed_hash) = PasswordHash::new(hash) else {
            // the hash is invalid
            return false;
        };
        match self
            .config
            .verify_password(password.as_bytes(), &parsed_hash)
        {
            Ok(()) => true,
            Err(Argon2Error::Password) => false,
            Err(e) => {
                tracing::warn!(algorithm = "argon2", "Password verification failed: {e}");
                false
            }
        }
    }
}
