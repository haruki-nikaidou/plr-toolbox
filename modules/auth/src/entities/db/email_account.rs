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
            .field("password_hash", &"[redacted]")
            .field("user_id", &self.user_id)
            .field("banned_at", &self.banned_at)
            .finish()
    }
}

impl EmailAccount {
    pub async fn find_by_email(
        conn: impl sqlx::PgExecutor<'_>,
        email: impl AsRef<str>,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM auth.email_account WHERE email = $1",
            email.as_ref()
        )
        .fetch_optional(conn)
        .await
    }

    pub async fn find_by_id(
        conn: impl sqlx::PgExecutor<'_>,
        id: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(Self, "SELECT * FROM auth.email_account WHERE id = $1", id)
            .fetch_optional(conn)
            .await
    }

    pub async fn find_by_user_id(
        conn: impl sqlx::PgExecutor<'_>,
        user_id: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM auth.email_account WHERE user_id = $1",
            user_id
        )
        .fetch_optional(conn)
        .await
    }

    pub async fn create(
        conn: impl sqlx::PgExecutor<'_>,
        new: CreateNewEmailAccount,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO auth.email_account (email, password_hash, user_id) 
            VALUES ($1, $2, $3) ON CONFLICT (email) 
            DO NOTHING 
            RETURNING *"#,
            new.email,
            new.password_hash,
            new.user_id,
        )
        .fetch_optional(conn)
        .await
    }

    pub async fn update_password(
        conn: impl sqlx::PgExecutor<'_>,
        id: Uuid,
        password_hash: String,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "UPDATE auth.email_account SET password_hash = $2 WHERE id = $1 RETURNING *",
            id,
            password_hash,
        )
        .fetch_optional(conn)
        .await
    }

    pub async fn update_email(
        conn: impl sqlx::PgExecutor<'_>,
        id: Uuid,
        email: String,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "UPDATE auth.email_account SET email = $2 WHERE id = $1 RETURNING *",
            id,
            email
        )
        .fetch_optional(conn)
        .await
    }
}

#[derive(Debug, Clone)]
pub struct CreateNewEmailAccount {
    pub email: String,
    pub password_hash: String,
    pub user_id: Uuid,
}
