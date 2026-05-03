use revolt_database::{util::reference::Reference, Database, Member, User};
use revolt_database::util::permissions::DatabasePermissionQuery;
use revolt_permissions::PermissionQuery;
use revolt_result::{create_error, Result};

use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use rocket_empty::EmptyResponse;

#[cfg(feature = "rocket")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket")]
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct DataAddBot {
    pub bot_id: String,
}

/// # Add Bot to Server
///
/// Adds a bot to a server on behalf of another bot (e.g. Otto inviting Quill).
/// Skips ManageServer permission check — this is a system operation.
/// No-ops if the bot is already a member.
#[openapi(tag = "Server Information")]
#[post("/<target>/add-bot", data = "<data>")]
pub async fn add_bot(
    db: &State<Database>,
    user: User,
    target: Reference<'_>,
    data: Json<DataAddBot>,
) -> Result<EmptyResponse> {
    if user.bot.is_none() {
        return Err(create_error!(NotPrivileged));
    }

    let server = target.as_server(db).await?;

    let mut query = DatabasePermissionQuery::new(db, &user).server(&server);
    if !query.are_we_a_member().await {
        return Err(create_error!(NotFound));
    }

    let bot_user = db.fetch_user(&data.bot_id).await?;

    // Ignore error if already a member
    let _ = Member::create(db, &server, &bot_user, None).await;

    Ok(EmptyResponse)
}
