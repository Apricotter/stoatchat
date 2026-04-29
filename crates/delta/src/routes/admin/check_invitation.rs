use revolt_database::Database;
use revolt_result::{create_error, Result};
use rocket::{serde::json::Json, State};
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct CheckResponse {
    pub valid: bool,
    pub email: String,
}

/// # Check Invitation
///
/// Validate an invitation code and return the associated email. Public endpoint used by the signup page.
#[openapi(tag = "Admin")]
#[get("/invitations/<code>/check")]
pub async fn check_invitation(
    db: &State<Database>,
    code: String,
) -> Result<Json<CheckResponse>> {
    let inv = db.fetch_invitation(&code).await.map_err(|_| create_error!(NotFound))?;
    if inv.used {
        return Err(create_error!(NotFound));
    }
    Ok(Json(CheckResponse { valid: true, email: inv.email }))
}
