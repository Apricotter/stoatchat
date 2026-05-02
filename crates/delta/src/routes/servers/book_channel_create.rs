use revolt_database::{util::reference::Reference, Channel, Database, User};
use revolt_database::util::permissions::DatabasePermissionQuery;
use revolt_permissions::PermissionQuery;
use revolt_result::{create_error, Result};
use revolt_models::v0;

use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[cfg(feature = "rocket")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket")]
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct BookChannelResponse {
    pub channel_id: String,
    pub channel_name: String,
}

/// # Create Book Processing Channel
///
/// Creates a per-book `book-processing-{slug}` channel on behalf of a bot user.
/// Skips ManageChannel permission check — this is a system operation.
#[openapi(tag = "Server Information")]
#[post("/<target>/book-channel", data = "<data>")]
pub async fn create_book_channel(
    db: &State<Database>,
    user: User,
    target: Reference<'_>,
    data: Json<v0::DataCreateServerChannel>,
) -> Result<Json<BookChannelResponse>> {
    let mut server = target.as_server(db).await?;

    // Bot must be a member of the server
    let mut query = DatabasePermissionQuery::new(db, &user).server(&server);
    if !query.are_we_a_member().await {
        return Err(create_error!(NotFound));
    }

    let channel_name = data.name.clone();

    let channel = Channel::create_server_channel(db, &mut server, data.into_inner(), true).await?;

    let channel_id = channel.id().to_string();

    Ok(Json(BookChannelResponse {
        channel_id,
        channel_name,
    }))
}
