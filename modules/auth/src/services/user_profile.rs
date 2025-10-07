use kanaeru::sqlx::DatabaseProcessor;

pub struct UserProfileService {
    pub db: DatabaseProcessor,
}
