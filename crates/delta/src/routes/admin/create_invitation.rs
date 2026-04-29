use revolt_database::{Database, Invitation, User};
use revolt_result::{create_error, Result};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};

const SIGNUP_BASE: &str = "https://hub2.apricotter.com/signup";

#[derive(Deserialize, JsonSchema)]
pub struct DataCreateInvitation {
    email: String,
    vertical: Option<String>,
    metadata: Option<serde_json::Value>,
    signup_base: Option<String>,
}

#[derive(Serialize, JsonSchema)]
pub struct InvitationResponse {
    pub code: String,
    pub email: String,
    pub signup_url: String,
}

/// # Create Invitation
///
/// Generate a single-use signup invitation for an email address. Requires privileged access.
#[openapi(tag = "Admin")]
#[post("/invitations", data = "<data>")]
pub async fn create_invitation(
    db: &State<Database>,
    user: User,
    data: Json<DataCreateInvitation>,
) -> Result<Json<InvitationResponse>> {
    if !user.privileged {
        return Err(create_error!(NotPrivileged));
    }

    let data = data.into_inner();
    let invitation = Invitation::new(
        data.email.clone(),
        user.id.clone(),
        data.vertical,
        data.metadata,
    );
    db.insert_invitation(&invitation).await?;

    let base = data.signup_base.as_deref().unwrap_or(SIGNUP_BASE);
    let signup_url = format!("{base}?code={}", invitation.code);

    Ok(Json(InvitationResponse {
        code: invitation.code,
        email: data.email,
        signup_url,
    }))
}
