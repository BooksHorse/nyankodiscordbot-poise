#![warn(clippy::str_to_string, clippy::all, clippy::nursery, clippy::pedantic)]
#![allow(clippy::unreadable_literal, clippy::unused_async)]
#![deny(clippy::perf)]
#[macro_use]
extern crate tracing;

use std::env;
use std::sync::Arc;
use std::time::Duration;

mod commands;

use commands::music;
// use commands::random;
// use commands::subject;
// use commands::then;

use commands::uwu;
use commands::vc;
use poise::async_trait;
use poise::serenity_prelude::prelude::*;
use poise::serenity_prelude::*;

//use songbird::serenity::SerenityInit;

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

    async fn voice_state_update(
        //no one in vc
        &self,
        ctx: poise::serenity_prelude::Context,
        _old_voice_state: Option<VoiceState>,
        new_voice_state: VoiceState,
    ) {
        tokio::time::sleep(Duration::from_secs(30)).await;
        if new_voice_state.channel_id == None {
            let manager = songbird::get(&ctx)
                .await
                .expect("Songbird Voice client placed in at initialisation.")
                .clone();
            let mut has_handler = false;
            let guild_id = new_voice_state.guild_id.unwrap();
            let channel_id = match manager.get(guild_id) {
                Some(call_mutex) => {
                    has_handler = true;
                    call_mutex.lock().await.current_channel().unwrap()
                }
                None => return,
            };
            let channels = guild_id.channels(&ctx).await.unwrap();
            let channel = channels.get(&ChannelId::from(channel_id.0)).unwrap();

            match channel.kind {
                ChannelType::Voice => {
                    if channel.members(&ctx.cache.clone()).unwrap().len() - 1 == 0 {
                        if has_handler {
                            if let Err(guild_channel) =
                                manager.remove(new_voice_state.guild_id.unwrap()).await
                            {
                                error!("{:?}", guild_channel);
                            }
                        }
                    } else {
                        //info!("voice channel user isn't 0: dont disconnect");
                        return;
                    }
                }
                _ => return,
            }
        }
        //println!("{:#?} || {:#?}",new,guild)
    }
}

pub struct Data {
    owner_id: UserId,
    reqwest_client: Arc<Mutex<reqwest::Client>>,
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
        ctx.clone().say("Unregistering commands globally").await?;
        Command::set_global_commands(ctx.clone(), vec![]).await?;
    } else {
        let guild = ctx.partial_guild().await;
        if let None = guild {
            ctx.say("Must be called in guild").await?;
        }
        ctx.say("Unregistering commands").await?;
        let a = guild.unwrap().clone().set_commands(ctx, vec![]).await?;
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
            // then::then_msg(),
            // then::then_text(),
            // then::enth_msg(),
            // then::enth_text(),
            music::volume(),
            music::play(),
            music::join(),
            music::queue(),
            music::skip(),
            music::stop(),
            music::leave(),
            music::r#loop(),
            poise::Command {
                subcommands: vec![
            vc::vcdisconnect(),
            vc::vcmove(),
                ],
                subcommand_required:true,
                ..vc::vc()
            },
            // subject::subject(),
            // random::random(),
            poise::Command {
                subcommands: vec![uwu::uwu(), uwu::owo(), uwu::uvu()],
                ..uwu::uwuify()
            },
        ],
        on_error: |error| Box::pin(on_error(error)),
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(".".into()),
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600)).into()),
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

    let framework = poise::Framework::builder()
        // .token(token)
        // .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .setup(move |ctx, _ready, framework| {
            let _shard_manager = framework.shard_manager();
            let _f = ctx.data.write();
            let client = reqwest::Client::new();
            Box::pin(async move {
                Ok(Data {
                    owner_id: UserId::new(313142847375278091),
                    reqwest_client: Arc::new(Mutex::new(client)),
                })
            })
        })
        .options(options)
        .build();
    // .client_settings(songbird::SerenityInit::register_songbird)
    // .run()
    // .await
    // .unwrap();

    let mut client = Client::builder(
        &token,
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler)
    .register_songbird()
    .framework(framework)
    .await?;
    client.start().await.unwrap();

    Ok(())
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(why) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", why);
            }
        }
        _ => {}
    }
}

use poise::serenity_prelude::model;
use songbird::SerenityInit;
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
