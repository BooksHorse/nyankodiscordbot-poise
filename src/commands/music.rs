use poise::{send_reply, serenity_prelude::Mentionable};
use songbird::{error::JoinError, input::Restartable};
use url::Url;

use crate::{Context, Error};
/// Play songs in YouTube
///
/// /play [URL,Search]
#[poise::command(prefix_command, slash_command, reuse_response, guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "URL or Search query"] query: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    // TODO:check if url or search

    let manager = songbird::get(ctx.discord())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if manager.get(ctx.guild().unwrap().id).is_some() {
        play_internal(ctx, query).await?;

        Ok(())
    } else {
        join_internal(ctx).await?;
        play_internal(ctx, query).await?;

        Ok(())
    }
}

pub async fn play_internal(ctx: Context<'_>, query: String) -> Result<(), Error> {
    ctx.defer().await?;
    // TODO:check if url or search

    let manager = songbird::get(ctx.discord())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(ctx.guild().unwrap().id) {
        let mut handler = handler_lock.lock().await;

        if let Ok(url) = Url::parse(&query) {
            if  ["youtube.com","youtu.be","www.youtube.com","m.youtube.com"].contains(&url.host_str().unwrap())
            {
                let source = match Restartable::ytdl(url, true).await {
                    Ok(source) => source,
                    Err(why) => {
                        send_reply(ctx, |f| {
                            f.content(format!("Err starting source: ```{:?}```", why))
                        })
                        .await?;
                        panic!("Err starting source: {:?}", why);
                    }
                };
                let dadw = handler
                    .enqueue_source(source.into())
                    .metadata()
                    .title
                    .clone()
                    .unwrap_or_else(|| "Unknown Title".to_owned());
                let channel_id = ctx
                    .guild()
                    .unwrap()
                    .voice_states
                    .get(&ctx.author().id)
                    .and_then(|voice_state| voice_state.channel_id);
                send_reply(ctx, |f| {
                    f.content(format!("Playing `{}` in <#{}>", dadw, channel_id.unwrap()))
                })
                .await?;
                info!("playing {} in {}", dadw, ctx.guild().unwrap().name);
            } else {
                send_reply(ctx, |f| f.content("Support only YouTube")).await?;
            }
        } else {
            let source = match Restartable::ytdl_search(query, true).await {
                Ok(source) => source,
                Err(why) => {
                    send_reply(ctx, |f| f.content(format!("Err search: ```{:?}```", why))).await?;
                    panic!("Err starting source: {:?}", why);
                }
            };

            let dadw = handler
                .enqueue_source(source.into())
                .metadata()
                .title
                .clone()
                .unwrap_or_else(|| "Unknown Title".to_owned());
            let channel_id = ctx
                .guild()
                .unwrap()
                .voice_states
                .get(&ctx.author().id)
                .and_then(|voice_state| voice_state.channel_id);
            send_reply(ctx, |f| {
                f.content(format!("Playing `{}` in <#{}>", dadw, channel_id.unwrap()))
            })
            .await?;
            info!("playing {} in {}", dadw, ctx.guild().unwrap().name);
        }
    }

    Ok(())
}

/// Joins your current voice channel
///
/// /join
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let channel_id = ctx
        .guild()
        .unwrap()
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => return Err(Box::new(JoinError::IllegalChannel)),
    };

    let manager = songbird::get(ctx.discord())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (handler_lock, success) = manager
        .join(ctx.guild().unwrap().id, channel_id.unwrap())
        .await;
    let _handler = handler_lock.lock().await;
    //   handler.add_global_event(
    //     Event::Track(TrackEvent::End),
    //     TrackEndNotifier {
    //         guild_id:guild,
    //         manager:manager.clone(),
    //         http: ctx.http.clone(),
    //     },
    // );

    if success.is_ok() {
        info!("{}", format!("Joined {}", connect_to.mention()));
    } else {
        error!("not in voice channel");
    };

    Ok(())
}

pub async fn join_internal(ctx: Context<'_>) -> Result<(), Error> {
    let channel_id = ctx
        .guild()
        .unwrap()
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => return Err(Box::new(JoinError::IllegalChannel)),
    };

    let manager = songbird::get(ctx.discord())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (handler_lock, success) = manager
        .join(ctx.guild().unwrap().id, channel_id.unwrap())
        .await;
    let _handler = handler_lock.lock().await;
    //   handler.add_global_event(
    //     Event::Track(TrackEvent::End),
    //     TrackEndNotifier {
    //         guild_id:guild,
    //         manager:manager.clone(),
    //         http: ctx.http.clone(),
    //     },
    // );

    if success.is_ok() {
        info!("{}", format!("Joined {}", connect_to.mention()));
    } else {
        error!("not in voice channel");
    };

    Ok(())
}

/// Leaves your current voice channel
///
/// /leaves
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx.discord())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(why) = manager.remove(guild_id).await {
            error!("{:?}", why);
            send_reply(ctx, |f| f.content(format!("Err: {:?}", why)))
                .await
                .unwrap();
        }

        send_reply(ctx, |f| f.content("Left voice channel"))
            .await
            .unwrap();
    } else {
        send_reply(ctx, |f| f.content("Not in voice channel"))
            .await
            .unwrap();
    }

    Ok(())
}

/// Skip songs n times
///
/// /skip [times]
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn skip(
    ctx: Context<'_>,
    #[description = "Skip count"] times: Option<i32>,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;
    //check if queue < skips
    let manager = songbird::get(ctx.discord())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let times = times.unwrap_or(1);
        for _i in 0..times {
            let _ = queue.skip();
        }
        send_reply(ctx, |f| {
            f.content(format!("Song skipped: {} in queue.", queue.len()))
        })
        .await?;
    } else {
        send_reply(ctx, |f| f.content("Not in voice channel")).await?;
    }

    Ok(())
}

/// Stops music
///
/// /stop
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx.discord())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        queue.stop();

        send_reply(ctx, |f| f.content("Queue cleared.")).await?;
    } else {
        send_reply(ctx, |f| f.content("Not in voice channel")).await?;
    }

    Ok(())
}

/// Lists current queue
///
/// /queue
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let guild_id = guild.id;
    let manager = songbird::get(ctx.discord())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        info!("{:?}", handler.queue().current_queue());
        if handler.queue().is_empty() {
            send_reply(ctx, |f| f.content("Queue is empty"))
                .await
                .unwrap();
            return Ok(());
        }
        let mut uwu: Vec<String> = vec![];
        for i in handler.queue().current_queue() {
            uwu.push(format!(
                "`{}` - {}",
                i.metadata().track.clone().unwrap_or_else(|| i
                    .metadata()
                    .title
                    .clone()
                    .unwrap_or_else(|| "Unknown Title".to_owned())),
                humantime::format_duration(i.metadata().duration.unwrap_or(std::time::Duration::new(0,0)))
            ));
        }
        send_reply(ctx, |f| f.content(uwu.join("\n")))
            .await
            .unwrap();
        Ok(())
    } else {
        send_reply(ctx, |f| f.content("Not in voice channel"))
            .await
            .unwrap();
        Ok(())
    }
}
