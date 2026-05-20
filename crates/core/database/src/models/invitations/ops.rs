use revolt_result::Result;

use crate::Invitation;

#[cfg(feature = "mongodb")]
mod mongodb;
mod reference;

#[async_trait]
pub trait AbstractInvitations: Sync + Send {
    async fn insert_invitation(&self, invitation: &Invitation) -> Result<()>;
    async fn fetch_invitation(&self, code: &str) -> Result<Invitation>;
    async fn fetch_all_invitations(&self) -> Result<Vec<Invitation>>;
    async fn fetch_invitation_by_user(&self, user_id: &str) -> Result<Invitation>;
    async fn mark_invitation_used(&self, code: &str, user_id: &str) -> Result<()>;
    async fn delete_invitation(&self, code: &str) -> Result<()>;
}
