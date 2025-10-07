use rand::Rng;
use time::PrimitiveDateTime;

#[derive(Clone, PartialEq, Eq, sqlx::FromRow, Debug)]
pub struct EmailOtp {
    pub id: i64,
    pub email: String,
    pub otp: String,
    pub has_been_used: bool,
    pub created_at: PrimitiveDateTime,
    pub reason: OtpReason,
}

#[derive(Clone, PartialEq, Eq, Debug, sqlx::Type)]
#[sqlx(type_name = "auth.otp_reason", rename_all = "snake_case")]
pub enum OtpReason {
    ChangePassword,
    ChangeEmailAddress,
    DeleteAccount,
}

pub fn generate_otp_code() -> String {
    let mut rng = rand::rng();
    let otp = format!("{:08}", rng.random_range(0..100000000));
    otp
}

impl EmailOtp {
    pub async fn find_by_email_valid(
        conn: impl sqlx::PgExecutor<'_>,
        email: impl AsRef<str>,
        time_after: PrimitiveDateTime,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, email, otp, has_been_used, created_at, reason as "reason: OtpReason"
            FROM auth.email_otp 
            WHERE email = $1 AND created_at > $2 AND has_been_used = FALSE
            "#,
            email.as_ref(),
            time_after,
        )
        .fetch_all(conn)
        .await
    }

    pub async fn delete_before(
        conn: impl sqlx::PgExecutor<'_>,
        time_before: PrimitiveDateTime,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM auth.email_otp WHERE created_at < $1",
            time_before,
        )
        .execute(conn)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn create(
        conn: impl sqlx::PgExecutor<'_>,
        new: CreateNewEmailOtp,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO auth.email_otp (email, otp, reason) VALUES ($1, $2, $3) 
            RETURNING id, email, otp, has_been_used, created_at, reason as "reason: OtpReason"
            "#,
            new.email,
            new.otp,
            new.reason as OtpReason,
        )
        .fetch_optional(conn)
        .await
    }

    pub async fn mark_as_used(
        conn: impl sqlx::PgExecutor<'_>,
        id: i64,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE auth.email_otp SET has_been_used = TRUE WHERE id = $1",
            id
        )
        .execute(conn)
        .await?;
        Ok(result.rows_affected() > 0)
    }
}

#[derive(Debug, Clone)]
pub struct CreateNewEmailOtp {
    pub email: String,
    pub otp: String,
    pub reason: OtpReason,
}
