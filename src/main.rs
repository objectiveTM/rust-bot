use anyhow::anyhow;
use serenity::builder::CreateEmbed;
use serenity::http::Http;
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
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
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