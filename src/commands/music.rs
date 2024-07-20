use std::sync::{Arc, Mutex};

use chrono::Duration;
use poise::{send_reply, serenity_prelude::{model::channel, ChannelType, Mentionable}, CreateReply};
use songbird::{error::{ControlError, JoinError}, input::{AuxMetadata, Compose}, tracks::{LoopState, Track}, typemap::TypeMapKey, Call};
use tokio::sync::MutexGuard;
use url::Url;

use crate::{Context, Error};

use youtube_dl::YoutubeDl;
use thiserror::Error;
#[derive(Error, Debug)]

#[error("Failed to set loop {onoff:?}: {why:?}")]
pub enum LoopError {
    LoopError {
    onoff:String,why:ControlError
}
}




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

    let manager = songbird::get(ctx.clone().serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if manager.get(ctx.guild_id().unwrap()).is_none() { // อยู่ในห้องอยู่แล้ว
        join_internal(ctx.clone()).await?;
    } 
    if let Ok(url) = Url::parse(&query) {
        if [
            "youtube.com",
            "youtu.be",
            "www.youtube.com",
            "m.youtube.com",
            "music.youtube.com"
        ]
        .contains(&url.host_str().unwrap()) {
            if query.contains("playlist") {
                let a = YoutubeDl::new(url).youtube_dl_path("yt-dlp").flat_playlist(true).run().unwrap().into_playlist().unwrap();
                let mut msgs:Vec<String> = vec![]; 
                let handle = send_reply(ctx, CreateReply::default().content("Loading Playlist")).await?;
                for f in a.entries.unwrap().iter() {
                    msgs.push(play_internal(ctx.clone(), f.url.to_owned().unwrap()).await?);
                    handle.edit(ctx, CreateReply::default().content("Loading Playlist").content(msgs.join("\n"))).await;
                    
                }
                return Ok(());
            };
        }
    }

        send_reply(ctx,CreateReply::default().content(format!("Playing {}",play_internal(ctx.clone(), query).await?))).await;


    
    Ok(())
}

struct SongData;

impl TypeMapKey for SongData {
    type Value = AuxMetadata;
}


pub async fn play_internal(ctx: Context<'_>, query: String) -> Result<String, Error> {
    ctx.defer().await?;
    // TODO:check if url or search

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(ctx.guild_id().unwrap()) {
        let mut handler = handler_lock.lock().await;

        if let Ok(url) = Url::parse(&query) {
            if [
                "youtube.com",
                "youtu.be",
                "www.youtube.com",
                "m.youtube.com",
                "music.youtube.com"
            ]
            .contains(&url.host_str().unwrap())
            {
                // let source = match Restartable::ytdl(url, true).await {
                //     Ok(source) => source,
                //     Err(why) => {
                //         send_reply(ctx, |f| {
                //             f.content(format!("Err starting source: ```{:?}```", why))
                //         })
                //         .await?;
                //         panic!("Err starting source: {:?}", why);
                //     }
                // };
                let mut source = songbird::input::YoutubeDl::new(ctx.data().reqwest_client.lock().await.to_owned(), url.to_string());
                
                
                
                let mut title= "Unknown Title".to_owned();
                let metadata = source.aux_metadata().await.inspect(|m| {
                    title=m.title.clone().unwrap_or("Unknown Title".to_owned());
                }).unwrap();
                // let track:Track = source.into();
                let mut channel_id;
                if handler.queue().current().is_none() {
                    let handle = handler.enqueue_input(source.into()).await;
                    let mut data = handle.typemap().write().await;

                    data.insert::<SongData>(metadata.clone());
                    
                    let _ = handle.play();
                } else {
                    let handle = handler.enqueue_with_preload(source.into(),Some(std::time::Duration::from_secs(10)));
                    let mut data = handle.typemap().write().await;
                data.insert::<SongData>(metadata.clone());
                };
                 channel_id = ctx
                    .guild()
                    .unwrap()
                    .voice_states
                    .get(&ctx.author().id)
                    .and_then(|voice_state| voice_state.channel_id);

                // if let Ok(data) = source.clone().aux_metadata().await {
                //     title=data.title.unwrap();
                // }
                
                // send_reply(ctx, CreateReply::default().content(format!("Playing `{}` in <#{}>", title, channel_id.unwrap())))
                // .await?;
                info!("playing {} in {}", title, ctx.guild().unwrap().name);
                return Ok(format!("`{}` by {}", title,metadata.channel.unwrap()));
            } else {
                return Ok("Support only YouTube URL".to_owned())
                // send_reply(ctx, CreateReply::default().content("Support only YouTube")).await?;
            }
            
        }
        else {
            return Ok("Support only Youtube URL".to_owned());
        }
    }
    else {
        return Ok("Failed to get guild.".to_owned())
    }

    // Ok(())
}

/// Joins your current voice channel
///
/// /join
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let channel_id = ctx
        .guild()
        .unwrap().clone()
        .voice_states
        .get(&ctx.author().clone().id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => return Err(Box::new(JoinError::NoCall)),
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    match manager.join(ctx.guild_id().unwrap(), channel_id.unwrap().clone()).await {
    Ok(k) => info!("{}", format!("Joined")),
    Err(k) =>  error!("Err!: {:?}",k),
    };
    // if success.is_ok() {
    //     info!("{}", format!("Joined {}", connect_to.mention()));
    // } else {
    //     error!("not in voice channel");
    // };

    Ok(())
}

pub async fn join_internal(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx
    .guild()
    .unwrap()
    .clone();
    let channel_id = guild
        .voice_states
        .get(&ctx.author().clone().id)
        .and_then(|voice_state| voice_state.channel_id);
    
        let connect_to = match channel_id {
            Some(channel) => channel,
            None => return Err(Box::new(JoinError::NoCall)),
        };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

        match manager.join(guild.id, channel_id.unwrap().clone()).await {
            Ok(k) => info!("{}", format!("Joined ")),
            Err(k) =>  error!("Err!: {:?}",k),
            };


    Ok(())
}

/// Leaves your current voice channel
///
/// /leaves
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().clone();
    let guild_id = guild.id;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(why) = manager.remove(guild_id).await {
            error!("{:?}", why);
            send_reply(ctx, CreateReply::default().content(format!("Err: {:?}", why)))
                .await
                .unwrap();
        }

        send_reply(ctx, CreateReply::default().content("Left voice channel"))
            .await
            .unwrap();
    } else {
        send_reply(ctx, CreateReply::default().content("Not in voice channel"))
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
    let guild = ctx.guild().unwrap().clone();
    let guild_id = guild.id;
    //check if queue < skips
    let manager = songbird::get(ctx.serenity_context())
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
        send_reply(ctx, CreateReply::default().content(format!("Song skipped: {} in queue.", queue.len()))
        )
        .await?;
    } else {
        send_reply(ctx, CreateReply::default().content("Not in voice channel")).await?;
    }

    Ok(())
}

/// Stops music
///
/// /stop
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().clone();
    let guild_id = guild.id;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        queue.stop();

        send_reply(ctx, CreateReply::default().content("Queue cleared.")).await?;
    } else {
        send_reply(ctx, CreateReply::default().content("Not in voice channel")).await?;
    }

    Ok(())
}

/// Lists current queue
///
/// /queue
#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().clone();
    let guild_id = guild.id;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        info!("{:?}", handler.queue().current_queue());
        if handler.queue().current_queue().is_empty() {
            send_reply(ctx, CreateReply::default().content("Queue is empty"))
                .await
                .unwrap();
            return Ok(());
        }
        let uwu: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
        let j = uwu.clone();
        for i in handler.queue().current_queue() {
            let s = j.clone();
                match i.typemap().read().await.get::<SongData>() {
                    Some(d) => uwu.lock().unwrap().push(format!("{} - {}",d.title.clone().unwrap_or("Unknown Title".to_owned()),d.channel.clone().unwrap_or("Unknown Channel".to_owned()))),
                    None => uwu.lock().unwrap().push("unk".to_owned()),
                }
                    
                }
        
        let sda =uwu.lock().unwrap().clone().join("\n");
        send_reply(ctx, CreateReply::default().content(sda))
            .await
            .unwrap();
        Ok(())
    } else {
        send_reply(ctx, CreateReply::default().content("Not in voice channel"))
            .await
            .unwrap();
        Ok(())
    }
}


#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn r#loop(ctx: Context<'_>,#[description = "true = loop , false = no loop"]r#loop:bool) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().clone();
    let guild_id = guild.id;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let d = match r#loop {
            true => LoopState::Infinite,
            false => LoopState::Finite(0)
        };
loop_internal(ctx,handler,d).await?;
Ok(())
    }
    else {
        send_reply(ctx, CreateReply::default().content("Not in voice channel"))
        .await
        .unwrap();
    Ok(())
    }

}

#[poise::command(prefix_command, slash_command, guild_only)]
pub async fn volume(ctx: Context<'_>,#[description = "value between 0 and 100"]volume:i8) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().clone();
    let guild_id = guild.id;
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let volume= volume.clamp(0, 100);
        handler.queue().current_queue().iter().for_each(|t| {t.set_volume(volume as f32/100.0);});
        ctx.say(format!("Set volume to {}",volume)).await;
    }
    else {
        send_reply(ctx, CreateReply::default().content("Not in voice channel"))
        .await
        .unwrap();
    }
Ok(())
}




async fn loop_internal(
    _ctx: Context<'_>,
    handler: MutexGuard<'_, Call>,
    onoff: LoopState,
  ) -> Result<String, LoopError> {
    match onoff {
      _Infinite => match handler.queue().current_queue().first().unwrap().enable_loop() {
        Ok(_) => Ok("loop on".to_string()),
        Err(why) => Err(LoopError::LoopError {
          onoff: "on".into(),
          why,
        }),
      },
      _ => match handler.queue().current_queue().first().unwrap().disable_loop() {
        Ok(_) => Ok("loop off".to_string()),
        Err(why) => Err(LoopError::LoopError {
          onoff: "off".into(),
          why,
        }),
      },
    }
  
    // let repeat = getloop(handler).await;
    // if repeat == Infinite {
    //     msg.channel_id.say(ctx,"loop on");
    // }
  }
  