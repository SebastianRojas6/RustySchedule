use crate::domain::user::model::User;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<User>>;
}
