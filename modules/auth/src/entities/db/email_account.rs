use time::PrimitiveDateTime;
use uuid::Uuid;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone, PartialEq, Eq, sqlx::FromRow, Zeroize, ZeroizeOnDrop)]
pub struct EmailAccount {
    #[zeroize(skip)]
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    #[zeroize(skip)]
    pub user_id: Uuid,
    #[zeroize(skip)]
    pub banned_at: Option<PrimitiveDateTime>,
}

impl core::fmt::Debug for EmailAccount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EmailAccount")
            .field("id", &self.id)
            .field("email", &self.email)
            .field("password_hash", "[redacted]")
            .field("user_id", &self.user_id)
            .field("banned_at", &self.banned_at)
            .finish()
    }
}