
use poise::{
    send_reply,
    serenity_prelude::ChannelType,
    CreateReply,
};
use futures::future;
use crate::{Context, Error};


///Move all people in old vc to new vc
#[poise::command(slash_command, prefix_command)]
pub async fn vcmove(
    ctx: Context<'_>,
    #[description = "Old Voice Channel"] old_vc: poise::serenity_prelude::GuildChannel,
    #[description = "New Voice Channel"] new_vc: poise::serenity_prelude::GuildChannel,
    
) -> Result<(), Error> {
    if old_vc.kind != ChannelType::Voice {
        let _ = ctx.say("Invalid Old Voice Channel").await;
        return Ok(());
    }
    if new_vc.kind != ChannelType::Voice {
        let _ = ctx.say("Invalid New Voice Channel").await;
        return Ok(());
    }
    if old_vc.id == new_vc.id {
        let _ = ctx.say("Old == New, Cannot move").await;
        return Ok(());
    }
    let count = old_vc.members(ctx.cache()).unwrap().len();
    future::join_all(old_vc.members(ctx.cache()).unwrap().iter().map(|member| {
        member.move_to_voice_channel(ctx, &new_vc)
    })).await;
    let _ = ctx.say(format!("Moved {count} users")).await;
    Ok(())
}

///Disconnect all people in vc
#[poise::command(slash_command, prefix_command)]
pub async fn vcdisconnect(
    ctx: Context<'_>,
    #[description = "Voice Channel"] vc: poise::serenity_prelude::GuildChannel,
    
) -> Result<(), Error> {
    if vc.kind != ChannelType::Voice {
        let _ = ctx.say("Invalid Channel").await;
        return Ok(());
    }
    let count = vc.members(ctx.cache()).unwrap().len();
    future::join_all(vc.members(ctx.cache()).unwrap().iter().map(|member| {
        member.disconnect_from_voice(ctx)
    })).await;
    let _ = ctx.say(format!("Disconnected {count} users")).await;
    Ok(())
}

///Commands for Voice Channel
#[poise::command(slash_command)]
pub async fn vc(ctx: Context<'_>) -> Result<(), Error> {
    send_reply(ctx, CreateReply::default().content("please select sub command")).await?;
    Ok(())
}