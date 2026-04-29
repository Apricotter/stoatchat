use futures::StreamExt;
use revolt_result::Result;

use crate::Invitation;
use crate::MongoDb;

use super::AbstractInvitations;

static COL: &str = "invitations";

#[async_trait]
impl AbstractInvitations for MongoDb {
    async fn insert_invitation(&self, invitation: &Invitation) -> Result<()> {
        query!(self, insert_one, COL, &invitation).map(|_| ())
    }

    async fn fetch_invitation(&self, code: &str) -> Result<Invitation> {
        query!(self, find_one_by_id, COL, code)?.ok_or_else(|| create_error!(NotFound))
    }

    async fn fetch_all_invitations(&self) -> Result<Vec<Invitation>> {
        Ok(self
            .col::<Invitation>(COL)
            .find(doc! {})
            .await
            .map_err(|_| create_database_error!("find", COL))?
            .filter_map(|s| async {
                if cfg!(debug_assertions) {
                    Some(s.unwrap())
                } else {
                    s.ok()
                }
            })
            .collect()
            .await)
    }

    async fn mark_invitation_used(&self, code: &str, user_id: &str) -> Result<()> {
        self.col::<Invitation>(COL)
            .update_one(
                doc! { "_id": code },
                doc! { "$set": { "used": true, "used_by": user_id } },
            )
            .await
            .map(|_| ())
            .map_err(|_| create_database_error!("update", COL))
    }

    async fn delete_invitation(&self, code: &str) -> Result<()> {
        query!(self, delete_one_by_id, COL, code).map(|_| ())
    }
}
