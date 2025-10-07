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
