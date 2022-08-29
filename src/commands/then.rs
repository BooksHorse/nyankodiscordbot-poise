use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, Colour};
use poise::{
    send_reply,
    serenity_prelude::{content_safe, ContentSafeOptions},
};
use thenconvert::Thenconvert;

#[poise::command(context_menu_command = "TH to EN", track_edits)]
pub async fn then_msg(
    ctx: Context<'_>,
    #[description = "Message to convert"] msg: serenity::Message,
) -> Result<(), Error> {
    let settings = ctx.guild_id().map_or_else(
        || {
            ContentSafeOptions::default()
                .clean_channel(false)
                .clean_role(false)
        },
        |guild_id| {
            //sanitize
            // By default roles, users, and channel mentions are cleaned.
            ContentSafeOptions::default()
                // We do not want to clean channal mentions as they
                // do not ping users.
                .clean_channel(false)
                // If it's a guild channel, we want mentioned users to be displayed
                // as their display name.
                .display_as_member_from(guild_id)
        },
    );

    let msgargs = content_safe(&ctx.discord().cache, msg.content, &settings, &[]);

    send_reply(ctx, |m| {
        m.embed(|e| {
            e.author(|a| {
                a.name(&ctx.author().name)
                    .icon_url(&ctx.author().avatar_url().unwrap_or_else(|| "".to_owned()))
            })
            .field(
                &msgargs,
                &msgargs
                    .th_to_en()
                    .replace('@', r#"\@"#)
                    .replace('`', r#"\`"#)
                    .replace('_', r#"\_"#)
                    .replace('*', r#"\*"#)
                    .replace('#', r#"\#"#),
                true,
            )
            .color(Colour::from_rgb(242, 153, 169))
            .description(format!("type: {:?}", "TH to EN"))
        })
    })
    .await?;

    Ok(())
}

#[poise::command(context_menu_command = "EN to TH", track_edits)]
pub async fn enth_msg(
    ctx: Context<'_>,
    #[description = "Message to convert"] msg: serenity::Message,
) -> Result<(), Error> {
    let settings = ctx.guild_id().map_or_else(
        || {
            ContentSafeOptions::default()
                .clean_channel(false)
                .clean_role(false)
        },
        |guild_id| {
            //sanitize
            // By default roles, users, and channel mentions are cleaned.
            ContentSafeOptions::default()
                // We do not want to clean channal mentions as they
                // do not ping users.
                .clean_channel(false)
                // If it's a guild channel, we want mentioned users to be displayed
                // as their display name.
                .display_as_member_from(guild_id)
        },
    );

    let msgargs = content_safe(&ctx.discord().cache, msg.content, &settings, &[]);

    send_reply(ctx, |m| {
        m.embed(|e| {
            e.author(|a| {
                a.name(&ctx.author().name)
                    .icon_url(&ctx.author().avatar_url().unwrap_or_else(|| "".to_owned()))
            })
            .field(
                &msgargs,
                &msgargs
                    .en_to_th()
                    .replace('@', r#"\@"#)
                    .replace('`', r#"\`"#)
                    .replace('_', r#"\_"#)
                    .replace('*', r#"\*"#)
                    .replace('#', r#"\#"#),
                true,
            )
            .color(Colour::from_rgb(242, 153, 169))
            .description(format!("type: {:?}", "EN to TH"))
        })
    })
    .await?;

    Ok(())
}
/// TH to EN Keyboard conversion
#[poise::command(slash_command, prefix_command, rename = "then", track_edits)]
pub async fn then_text(
    ctx: Context<'_>,
    #[description = "Text to convert"] text: String,
) -> Result<(), Error> {
    let settings = ctx.guild_id().map_or_else(
        || {
            ContentSafeOptions::default()
                .clean_channel(false)
                .clean_role(false)
        },
        |guild_id| {
            //sanitize
            // By default roles, users, and channel mentions are cleaned.
            ContentSafeOptions::default()
                // We do not want to clean channal mentions as they
                // do not ping users.
                .clean_channel(false)
                // If it's a guild channel, we want mentioned users to be displayed
                // as their display name.
                .display_as_member_from(guild_id)
        },
    );

    let msgargs = content_safe(&ctx.discord().cache, text, &settings, &[]);

    send_reply(ctx, |m| {
        m.embed(|e| {
            e.author(|a| {
                a.name(&ctx.author().name)
                    .icon_url(&ctx.author().avatar_url().unwrap_or_else(|| "".to_owned()))
            })
            .field(
                &msgargs,
                &msgargs
                    .th_to_en()
                    .replace('@', r#"\@"#)
                    .replace('`', r#"\`"#)
                    .replace('_', r#"\_"#)
                    .replace('*', r#"\*"#)
                    .replace('#', r#"\#"#),
                true,
            )
            .color(Colour::from_rgb(242, 153, 169))
            .description(format!("type: {:?}", "TH to EN"))
        })
    })
    .await?;

    Ok(())
}

/// EN to TH Keyboard conversion
#[poise::command(slash_command, prefix_command, track_edits, rename = "enth")]
pub async fn enth_text(
    ctx: Context<'_>,
    #[description = "Text to convert"] text: String,
) -> Result<(), Error> {
    let settings = ctx.guild_id().map_or_else(
        || {
            ContentSafeOptions::default()
                .clean_channel(false)
                .clean_role(false)
        },
        |guild_id| {
            //sanitize
            // By default roles, users, and channel mentions are cleaned.
            ContentSafeOptions::default()
                // We do not want to clean channal mentions as they
                // do not ping users.
                .clean_channel(false)
                // If it's a guild channel, we want mentioned users to be displayed
                // as their display name.
                .display_as_member_from(guild_id)
        },
    );
    let msgargs = content_safe(&ctx.discord().cache, text, &settings, &[]);

    send_reply(ctx, |m| {
        m.embed(|e| {
            e.author(|a| {
                a.name(&ctx.author().name)
                    .icon_url(&ctx.author().avatar_url().unwrap_or_else(|| "".to_owned()))
            })
            .field(
                &msgargs,
                &msgargs
                    .en_to_th()
                    .replace('@', r#"\@"#)
                    .replace('`', r#"\`"#)
                    .replace('_', r#"\_"#)
                    .replace('*', r#"\*"#)
                    .replace('#', r#"\#"#),
                true,
            )
            .color(Colour::from_rgb(242, 153, 169))
            .description(format!("type: {:?}", "EN to TH"))
        })
    })
    .await?;

    Ok(())
}
