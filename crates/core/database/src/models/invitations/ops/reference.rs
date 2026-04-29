use revolt_result::Result;

use crate::Invitation;
use crate::ReferenceDb;

use super::AbstractInvitations;

#[async_trait]
impl AbstractInvitations for ReferenceDb {
    async fn insert_invitation(&self, invitation: &Invitation) -> Result<()> {
        let mut invitations = self.invitations.lock().await;
        if invitations.contains_key(&invitation.code) {
            Err(create_database_error!("insert", "invitation"))
        } else {
            invitations.insert(invitation.code.clone(), invitation.clone());
            Ok(())
        }
    }

    async fn fetch_invitation(&self, code: &str) -> Result<Invitation> {
        let invitations = self.invitations.lock().await;
        invitations
            .get(code)
            .cloned()
            .ok_or_else(|| create_error!(NotFound))
    }

    async fn fetch_all_invitations(&self) -> Result<Vec<Invitation>> {
        let invitations = self.invitations.lock().await;
        Ok(invitations.values().cloned().collect())
    }

    async fn mark_invitation_used(&self, code: &str, user_id: &str) -> Result<()> {
        let mut invitations = self.invitations.lock().await;
        if let Some(inv) = invitations.get_mut(code) {
            inv.used = true;
            inv.used_by = Some(user_id.to_string());
            Ok(())
        } else {
            Err(create_error!(NotFound))
        }
    }

    async fn delete_invitation(&self, code: &str) -> Result<()> {
        let mut invitations = self.invitations.lock().await;
        if invitations.remove(code).is_some() {
            Ok(())
        } else {
            Err(create_error!(NotFound))
        }
    }
}
