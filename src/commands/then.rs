use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use poise::{
    send_reply,
    serenity_prelude::{content_safe, ContentSafeOptions},
};

#[poise::command(context_menu_command = "TH to EN", reuse_response, guild_only)]
pub async fn then_msg(
    ctx: Context<'_>,
    #[description = "Message to convert"] msg: serenity::Message,
) -> Result<(), Error> {
    let settings = if let Some(guild_id) = ctx.guild_id() {
        //sanitize
        // By default roles, users, and channel mentions are cleaned.
        ContentSafeOptions::default()
            // We do not want to clean channal mentions as they
            // do not ping users.
            .clean_channel(false)
            // If it's a guild channel, we want mentioned users to be displayed
            // as their display name.
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let msgargs = content_safe(&ctx.discord().cache, msg.content, &settings, &[]);

    send_reply(ctx, |f| {
        f.content(
            thenconvert::th_to_en(&msgargs)
                .unwrap()
                .replace('@', r#"\@"#)
                .replace('`', r#"\`"#)
                .replace('_', r#"\_"#)
                .replace('*', r#"\*"#)
                .replace('#', r#"\#"#),
        )
    })
    .await?;

    Ok(())
}

#[poise::command(context_menu_command = "EN to TH", reuse_response, guild_only)]
pub async fn enth_msg(
    ctx: Context<'_>,
    #[description = "Message to convert"] msg: serenity::Message,
) -> Result<(), Error> {
    let settings = if let Some(guild_id) = ctx.guild_id() {
        //sanitize
        // By default roles, users, and channel mentions are cleaned.
        ContentSafeOptions::default()
            // We do not want to clean channal mentions as they
            // do not ping users.
            .clean_channel(false)
            // If it's a guild channel, we want mentioned users to be displayed
            // as their display name.
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let msgargs = content_safe(&ctx.discord().cache, msg.content, &settings, &[]);

    send_reply(ctx, |f| {
        f.content(
            thenconvert::en_to_th(&msgargs)
                .unwrap()
                .replace('@', r#"\@"#)
                .replace('`', r#"\`"#)
                .replace('_', r#"\_"#)
                .replace('*', r#"\*"#)
                .replace('#', r#"\#"#),
        )
    })
    .await?;

    Ok(())
}
/// TH to EN Keyboard conversion
#[poise::command(slash_command, reuse_response, guild_only, rename = "then")]
pub async fn then_text(
    ctx: Context<'_>,
    #[description = "Text to convert"] text: String,
) -> Result<(), Error> {
    let settings = if let Some(guild_id) = ctx.guild_id() {
        //sanitize
        // By default roles, users, and channel mentions are cleaned.
        ContentSafeOptions::default()
            // We do not want to clean channal mentions as they
            // do not ping users.
            .clean_channel(false)
            // If it's a guild channel, we want mentioned users to be displayed
            // as their display name.
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let msgargs = content_safe(&ctx.discord().cache, text, &settings, &[]);

    send_reply(ctx, |f| {
        f.content(
            thenconvert::th_to_en(&msgargs)
                .unwrap()
                .replace('@', r#"\@"#)
                .replace('`', r#"\`"#)
                .replace('_', r#"\_"#)
                .replace('*', r#"\*"#)
                .replace('#', r#"\#"#),
        )
    })
    .await?;

    Ok(())
}

/// EN to TH Keyboard conversion
#[poise::command(slash_command, reuse_response, guild_only, rename = "enth")]
pub async fn enth_text(
    ctx: Context<'_>,
    #[description = "Text to convert"] text: String,
) -> Result<(), Error> {
    let settings = if let Some(guild_id) = ctx.guild_id() {
        //sanitize
        // By default roles, users, and channel mentions are cleaned.
        ContentSafeOptions::default()
            // We do not want to clean channal mentions as they
            // do not ping users.
            .clean_channel(false)
            // If it's a guild channel, we want mentioned users to be displayed
            // as their display name.
            .display_as_member_from(guild_id)
    } else {
        ContentSafeOptions::default()
            .clean_channel(false)
            .clean_role(false)
    };

    let msgargs = content_safe(&ctx.discord().cache, text, &settings, &[]);

    send_reply(ctx, |f| {
        f.content(
            thenconvert::en_to_th(&msgargs)
                .unwrap()
                .replace('@', r#"\@"#)
                .replace('`', r#"\`"#)
                .replace('_', r#"\_"#)
                .replace('*', r#"\*"#)
                .replace('#', r#"\#"#),
        )
    })
    .await?;

    Ok(())
}
