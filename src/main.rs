use std::sync::Arc;

use anyhow::anyhow;
use serenity::builder::CreateEmbed;
use serenity::client::bridge::gateway::ShardRunnerInfo;
use serenity::http::Http;
use serenity::model::prelude::{GuildId, Interaction, InteractionResponseType};
use serenity::{async_trait, model::prelude::Embed};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot;
// #![feature(async_fn_in_trait)]

//  trait SendMessage{
//     // pub async fn send_message<'a, F>(self, http: impl AsRef<Http>, f: F) -> Result<Message>;
//     async fn a();
// }
// impl ctx{}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
                // let embed = CreateEmbed{..Default::default()};
                msg.channel_id.send_message(&ctx.http, |msg|{ msg.embed(|e|{
                    e.title("title").description("description")
            }) }).await.unwrap();
        }

        if msg.content == "!ping" {
            // let a = ctx.http
            
            // msg.channel_id.say(&ctx.http, ctx.http).await.unwrap();
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let guild_id = GuildId(850364325834391582);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| { command.name("hello").description("Say hello") })
        }).await.unwrap();
        info!("{} is connected!", ready.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "hello" => {format!("{:?}", ctx.http)},
                command => unreachable!("Unknown command: {}", command),
            };

            let create_interaction_response =
                command.create_interaction_response(&ctx.http, |response| {
                    response
                        // .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(response_content))
                });

            if let Err(why) = create_interaction_response.await {
                eprintln!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
