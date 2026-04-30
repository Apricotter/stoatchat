use authifier::models::Session;
use once_cell::sync::Lazy;
use regex::Regex;
use revolt_database::{Channel, Database, Member, Message, Server, User, AMQP};
use revolt_models::v0::{self, DataCreateServer, DataCreateServerChannel, LegacyServerChannelType, MessageAuthor};
use revolt_result::{create_error, Result};
use ulid::Ulid;

use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Regex for valid usernames
///
/// Block zero width space
/// Block lookalike characters
pub static RE_USERNAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\p{L}|[\d_.-])+$").unwrap());

/// # New User Data
#[derive(Validate, Serialize, Deserialize, JsonSchema)]
pub struct DataOnboard {
    /// New username which will be used to identify the user on the platform
    #[validate(length(min = 2, max = 32), regex = "RE_USERNAME")]
    username: String,
    /// Entry code required when the server has registration gating enabled
    entry_code: Option<String>,
}

/// # Complete Onboarding
///
/// This sets a new username, completes onboarding and allows a user to start using Revolt.
#[openapi(tag = "Onboarding")]
#[post("/complete", data = "<data>")]
pub async fn complete(
    db: &State<Database>,
    amqp: &State<AMQP>,
    session: Session,
    user: Option<User>,
    data: Json<DataOnboard>,
) -> Result<Json<v0::User>> {
    if user.is_some() {
        return Err(create_error!(AlreadyOnboarded));
    }

    let data = data.into_inner();
    data.validate().map_err(|error| {
        create_error!(FailedValidation {
            error: error.to_string()
        })
    })?;

    let code = data.entry_code.as_deref().ok_or_else(|| create_error!(InvalidEntryCode))?;
    let invitation = db.fetch_invitation(code).await.map_err(|_| create_error!(InvalidEntryCode))?;
    if invitation.used {
        return Err(create_error!(InvalidEntryCode));
    }

    let user = User::create(db, data.username.clone(), session.user_id.clone(), None).await?;
    db.mark_invitation_used(code, &user.id).await?;

    // Create the client's dedicated studio server
    let (mut server, _) = Server::create(
        db,
        DataCreateServer {
            name: format!("{}'s Studio", user.username),
            description: None,
            nsfw: None,
        },
        &user,
        false,
    )
    .await?;

    // Client-named channel comes first (their primary general channel)
    let _ = Channel::create_server_channel(
        db,
        &mut server,
        DataCreateServerChannel {
            channel_type: LegacyServerChannelType::Text,
            name: user.username.to_lowercase(),
            ..Default::default()
        },
        true,
    )
    .await;

    // Bot channel — where the onboarding assistant lives
    let bot_channel = Channel::create_server_channel(
        db,
        &mut server,
        DataCreateServerChannel {
            channel_type: LegacyServerChannelType::Text,
            name: "bot".to_string(),
            ..Default::default()
        },
        true,
    )
    .await;

    // Remaining studio channels
    for name in &["strategies", "content", "approvals", "review", "documents"] {
        let _ = Channel::create_server_channel(
            db,
            &mut server,
            DataCreateServerChannel {
                channel_type: LegacyServerChannelType::Text,
                name: name.to_string(),
                ..Default::default()
            },
            true,
        )
        .await;
    }

    // Add the client as a member of their studio
    let _ = Member::create(db, &server, &user, None).await;

    // Add the system bot and send the welcome message in the bot channel
    if let (Ok(bot_token), Ok(bot_channel)) = (std::env::var("APRICOTTER_BOT_TOKEN"), bot_channel) {
        if let Ok(bot) = db.fetch_bot_by_token(&bot_token).await {
            if let Ok(bot_user) = db.fetch_user(&bot.id).await {
                let _ = Member::create(db, &server, &bot_user, None).await;

                let bot_v0: v0::User = bot_user.clone().into(db, Some(&bot_user)).await;
                let mut message = Message {
                    id: Ulid::new().to_string(),
                    channel: bot_channel.id().to_string(),
                    author: bot_user.id.clone(),
                    content: Some(format!(
                        "Hello {}! I'm your personal assistant. I'd like to help you complete onboarding.",
                        user.username
                    )),
                    ..Default::default()
                };
                let _ = message
                    .send(
                        db,
                        Some(amqp),
                        MessageAuthor::User(&bot_v0.clone()),
                        Some(bot_v0),
                        None,
                        &bot_channel,
                        false,
                    )
                    .await;
            }
        }
    }

    Ok(Json(user.into_self(false).await))
}
