use authifier::models::Session;
use revolt_database::{Channel, Database, Member, Message, Server, User, AMQP};
use revolt_models::v0::{self, DataCreateServer, DataCreateServerChannel, LegacyServerChannelType, MessageAuthor};
use revolt_result::{create_error, Result};
use ulid::Ulid;

use rocket::{serde::json::Json, State};
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct HealResult {
    pub healed: bool,
    pub server_id: Option<String>,
}

/// # Heal Account
///
/// Creates the client's studio server and channels if they don't already have one.
/// Safe to call multiple times — does nothing if the studio already exists.
#[openapi(tag = "Onboarding")]
#[post("/heal")]
pub async fn heal(
    db: &State<Database>,
    amqp: &State<AMQP>,
    _session: Session,
    user: User,
) -> Result<Json<HealResult>> {
    // Check if user already has a studio server
    let memberships = db.fetch_all_memberships(&user.id).await?;
    let server_ids: Vec<String> = memberships.iter().map(|m| m.id.server.clone()).collect();

    if !server_ids.is_empty() {
        let servers = db.fetch_servers(&server_ids).await?;
        if let Some(existing) = servers.iter().find(|s| s.name.to_lowercase().contains("studio")) {
            return Ok(Json(HealResult {
                healed: false,
                server_id: Some(existing.id.clone()),
            }));
        }
    }

    // No studio found — create one
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

    let server_id = server.id.clone();

    let _ = Channel::create_server_channel(
        db,
        &mut server,
        DataCreateServerChannel {
            channel_type: LegacyServerChannelType::Text,
            name: "onboarding".to_string(),
            ..Default::default()
        },
        true,
    )
    .await;

    let bot_channel = Channel::create_server_channel(
        db,
        &mut server,
        DataCreateServerChannel {
            channel_type: LegacyServerChannelType::Text,
            name: "assistant".to_string(),
            ..Default::default()
        },
        true,
    )
    .await;

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

    let _ = Member::create(db, &server, &user, None).await;

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

    Ok(Json(HealResult {
        healed: true,
        server_id: Some(server_id),
    }))
}
