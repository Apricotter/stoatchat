use futures::future::join_all;
use revolt_database::{Database, User};
use revolt_models::v0;
use revolt_result::{create_error, Result};

use rocket::{serde::json::Json, State};

/// # List All Users
///
/// Returns all users on the instance. Requires privileged access.
#[openapi(tag = "User Information")]
#[get("/list")]
pub async fn list_all(db: &State<Database>, user: User) -> Result<Json<Vec<v0::User>>> {
    if !user.privileged {
        return Err(create_error!(NotPrivileged));
    }

    let users = db.fetch_all_users().await?;
    let result = join_all(users.into_iter().map(|u| u.into_self(false))).await;
    Ok(Json(result))
}
