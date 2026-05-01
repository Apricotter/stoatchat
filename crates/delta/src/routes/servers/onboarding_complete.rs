use revolt_database::{
    util::reference::Reference, Database, FieldsServer, PartialServer, User,
};
use revolt_result::{create_error, Result};
use rocket::State;

/// # Complete Onboarding
///
/// Called by Otto (bot) after the client has confirmed their profile.
/// Sets onboarding_complete on the server.
#[openapi(tag = "Server Information")]
#[post("/<target>/complete-onboarding")]
pub async fn complete_onboarding(
    db: &State<Database>,
    user: User,
    target: Reference<'_>,
) -> Result<()> {
    if user.bot.is_none() {
        return Err(create_error!(NotPrivileged));
    }

    let mut server = target.as_server(db).await?;
    server
        .update(
            db,
            PartialServer {
                onboarding_complete: Some(true),
                ..Default::default()
            },
            Vec::<FieldsServer>::new(),
        )
        .await?;

    Ok(())
}
