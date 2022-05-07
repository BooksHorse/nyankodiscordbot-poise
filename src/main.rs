#![warn(clippy::str_to_string)]
#[macro_use]
extern crate tracing;

use std::env;

mod commands;

use commands::music;
use commands::subject;
use commands::then;

use poise::serenity_prelude::UserId;
use serenity::http::Http;


use songbird::serenity::SerenityInit;



use serenity::model::gateway::GatewayIntents;

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let options = poise::FrameworkOptions {
        commands: vec![
            register(),
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
        ],
        on_error: |error| Box::pin(on_error(error)),
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(".".into()),
            ..Default::default()
        },
        ..Default::default()
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
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                    owner_id: UserId(313142847375278091),
                })
            })
        })
        .options(options)
        .client_settings(|f| f.register_songbird())
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
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}
