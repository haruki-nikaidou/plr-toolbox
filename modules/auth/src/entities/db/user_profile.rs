use time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, sqlx::FromRow, Debug)]
pub struct UserProfile {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

impl UserProfile {
    pub async fn find_by_user_id(
        conn: impl sqlx::PgExecutor<'_>,
        user_id: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "SELECT * FROM auth.user_profile WHERE id = $1",
            user_id
        )
        .fetch_optional(conn)
        .await
    }

    pub async fn create(
        conn: impl sqlx::PgExecutor<'_>,
        new: CreateNewUserProfile,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "INSERT INTO auth.user_profile (name, email) VALUES ($1, $2) RETURNING *",
            new.name,
            new.email
        )
        .fetch_optional(conn)
        .await
    }

    pub async fn update_name(
        conn: impl sqlx::PgExecutor<'_>,
        id: Uuid,
        name: String,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "UPDATE auth.user_profile SET name = $2 WHERE id = $1 RETURNING *",
            id,
            name
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
            "UPDATE auth.user_profile SET email = $2 WHERE id = $1 RETURNING *",
            id,
            email
        )
        .fetch_optional(conn)
        .await
    }
}

#[derive(Debug, Clone)]
pub struct CreateNewUserProfile {
    pub name: String,
    pub email: String,
}
