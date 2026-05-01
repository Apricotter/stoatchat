use revolt_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    AbstractGreetings, Database, User,
};
use revolt_permissions::PermissionQuery;
use revolt_result::{create_error, Result};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};

#[cfg(feature = "rocket")]
use rocket_okapi::okapi::schemars;
#[cfg(feature = "rocket")]
use rocket_okapi::okapi::schemars::JsonSchema;

/// Server greeting response
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GreetingResponse {
    /// The vertical key (e.g. "author", "home_services", "default")
    pub vertical: String,
    /// Greeting message template; {username} is substituted by the caller if known
    pub message: String,
    /// Intake metadata captured at invite time (arbitrary JSON object)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// # Fetch Server Greeting
///
/// Returns the onboarding greeting message for a server's client vertical.
/// Used by Otto to send the welcome message in #start-here.
#[openapi(tag = "Server Information")]
#[get("/<target>/greeting")]
pub async fn fetch_greeting(
    db: &State<Database>,
    user: User,
    target: Reference<'_>,
) -> Result<Json<GreetingResponse>> {
    let server = target.as_server(db).await?;
    let mut query = DatabasePermissionQuery::new(db, &user).server(&server);
    if !query.are_we_a_member().await {
        return Err(create_error!(NotFound));
    }

    let vertical = server.vertical.as_deref().unwrap_or("default");
    let greeting = db.fetch_greeting(Some(vertical)).await?;
    let metadata = server
        .intake_metadata
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok());

    Ok(Json(GreetingResponse {
        vertical: greeting.vertical,
        message: greeting.message,
        metadata,
    }))
}
