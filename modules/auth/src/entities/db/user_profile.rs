use time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, sqlx::FromRow, Debug)]
pub struct UserProfile {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}
