use uuid::Uuid;

#[derive(Clone, PartialEq, Eq)]
pub struct AccessToken(String);

impl core::fmt::Debug for AccessToken {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AccessToken([redacted])")
    }
}

impl AccessToken {
    pub fn new(token: String) -> Self {
        Self(token)
    }
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl PartialEq<str> for AccessToken {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl AsRef<str> for AccessToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<String> for AccessToken {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}

pub struct RefreshToken(String);

impl core::fmt::Debug for RefreshToken {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "RefreshToken([redacted])")
    }
}

impl RefreshToken {
    pub fn new(token: String) -> Self {
        Self(token)
    }
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl PartialEq<str> for RefreshToken {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl AsRef<str> for RefreshToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<String> for RefreshToken {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}

#[derive(Clone)]
pub struct AccessTokenClaims {
    /// User ID
    pub sub: Uuid,
    pub exp: i64,
    pub iss: String,
    pub aud: String
}

pub struct RefreshTokenClaims {
    /// Session ID
    pub sub: Uuid,
    pub exp: i64,
    pub iss: String,
    pub aud: String,
}