use revolt_database::{Database, User};
use revolt_result::{create_error, Result};
use rocket::{serde::json::Json, State};
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct DeleteResponse {
    pub deleted: bool,
}

/// # Delete Invitation
///
/// Revoke an invitation by its code. Requires privileged access.
#[openapi(tag = "Admin")]
#[delete("/invitations/<code>")]
pub async fn delete_invitation(
    db: &State<Database>,
    user: User,
    code: String,
) -> Result<Json<DeleteResponse>> {
    if !user.privileged {
        return Err(create_error!(NotPrivileged));
    }

    db.delete_invitation(&code).await?;
    Ok(Json(DeleteResponse { deleted: true }))
}
