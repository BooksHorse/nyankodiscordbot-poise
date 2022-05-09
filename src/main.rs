#![warn(clippy::str_to_string, clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::unreadable_literal, clippy::unused_async)]
#![deny(clippy::perf)]
#[macro_use]
extern crate tracing;

use std::env;
use std::sync::Arc;

mod commands;

use commands::music;
use commands::random;
use commands::subject;
use commands::then;

use commands::uwu;
use poise::async_trait;
use poise::serenity_prelude::ApplicationCommand;
use poise::serenity_prelude::EventHandler;
use poise::serenity_prelude::Ready;
use poise::serenity_prelude::RwLock;
use poise::serenity_prelude::ShardManager;
use poise::serenity_prelude::TypeMapKey;
use poise::serenity_prelude::UserId;
use serenity::http::Http;

//use songbird::serenity::SerenityInit;

use serenity::model::gateway::GatewayIntents;
use std::sync::mpsc::channel;


struct KillCommand;

impl TypeMapKey for KillCommand {
  type Value = Arc<RwLock<ShardManager>>;
}

#[async_trait]
impl EventHandler for Handler {

// async fn ready(&self,ctx:poise::serenity_prelude::Context,data_about_bot:Ready) {


//     tokio::spawn(async move {
//         wait_shutdown().await.unwrap();
//         ctx.shard.shutdown_clean();
//       });

// }

}

pub struct Data {
    owner_id: UserId,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Handler;

async fn is_owner(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.author().id == ctx.data().owner_id)
}

#[poise::command(prefix_command, check = "is_owner", hide_in_help)]
async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), Error> {
    poise::samples::register_application_commands(ctx, global).await?;
    Ok(())
}

#[poise::command(prefix_command, check = "is_owner", hide_in_help)]
async fn unregister(ctx: Context<'_>, #[flag] global: bool) -> Result<(), Error> {
    if global {
        ctx.say("Unregistering commands globally").await?;
        ApplicationCommand::set_global_application_commands(ctx.discord(), |b| {
            b.set_application_commands(vec![])
        })
        .await?;
    } else {
        let guild = match ctx.guild() {
            Some(x) => x,
            None => {
                ctx.say("Must be called in guild").await?;
                return Ok(());
            }
        };
        ctx.say("Unregistering commands").await?;
        guild
            .set_application_commands(ctx.discord(), |b| b.set_application_commands(vec![]))
            .await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let options = poise::FrameworkOptions {
        commands: vec![
            register(),
            unregister(),
            then::then_msg(),
            then::then_text(),
            then::enth_msg(),
            then::enth_text(),
            music::play(),
            music::join(),
            music::queue(),
            music::skip(),
            music::stop(),
            music::leave(),
            subject::subject(),
            random::random(),
            poise::Command {
                subcommands: vec![uwu::uwu(), uwu::owo(), uwu::uvu()],
                ..uwu::uwuify()
            },
        ],
        on_error: |error| Box::pin(on_error(error)),
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(".".into()),
            ..poise::PrefixFrameworkOptions::default()
        },
        ..poise::FrameworkOptions::default()
    };

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new(&token);

    let _bot_id = match http.get_current_application_info().await {
        Ok(info) => info.id,
        Err(why) => panic!("Could not access application info: {:?}", why),
    };




  


     poise::Framework::build()
        .token(token)
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .user_data_setup(move |ctx, _ready, framework| {
            let shard_manager = framework.shard_manager();
            let f = ctx.data.write();
            Box::pin(async move {
                Ok(Data {
                    owner_id: UserId(313142847375278091),
                })
            })
        })
        .options(options)
        .client_settings(songbird::SerenityInit::register_songbird)
        .run()
        .await
        .unwrap();
    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(why) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", why);
            }
        }
    }
}

#[cfg(target_os = "linux")]
use tokio::signal::unix::{signal, SignalKind};
#[cfg(target_os = "linux")]
async fn wait_shutdown() -> Result<(), Box<dyn std::error::Error>> {
  let mut a = signal(SignalKind::interrupt())?;
  let mut b = signal(SignalKind::terminate())?;
  let mut c = signal(SignalKind::quit())?;
  tokio::select! {
      _ = a.recv() => {Ok(())},
      _ = b.recv() => {Ok(())},
      _ = c.recv() => {Ok(())}
  }
}

#[cfg(target_os = "windows")]
async fn wait_shutdown() -> Result<(), Box<dyn std::error::Error>> {
  tokio::signal::ctrl_c().await.unwrap(); //no sigterm handler
  Ok(())
}
