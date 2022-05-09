use owoify_rs::{Owoifiable, OwoifyLevel};
use poise::{
    send_reply,
    serenity_prelude::{content_safe, Colour, ContentSafeOptions},
};

use crate::{Context, Error};
///Convert text to UwU
#[poise::command(slash_command)]
pub async fn uwuify(ctx: Context<'_>) -> Result<(), Error> {
    send_reply(ctx, |m| m.content("hehe")).await?;
    Ok(())
}

///Minimum level of uwuness
#[poise::command(slash_command, prefix_command)]
pub async fn uwu(
    ctx: Context<'_>,
    #[description = "Text to UwUify"] text: String,
) -> Result<(), Error> {
    uwoh(ctx, text, OwoifyLevel::Uwu).await?;
    Ok(())
}

///Medium level of uwuness
#[poise::command(slash_command, prefix_command)]
pub async fn owo(
    ctx: Context<'_>,
    #[description = "Text to OwOify"] text: String,
) -> Result<(), Error> {
    uwoh(ctx, text, OwoifyLevel::Owo).await?;
    Ok(())
}
///Maximum level of uwuness
#[poise::command(slash_command, prefix_command)]
pub async fn uvu(
    ctx: Context<'_>,
    #[description = "Text to UvUify"] text: String,
) -> Result<(), Error> {
    uwoh(ctx, text, OwoifyLevel::Uvu).await?;
    Ok(())
}

async fn uwoh(ctx: Context<'_>, text: String, owoify_level: OwoifyLevel) -> Result<(), Error> {
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
            .field(&msgargs, msgargs.owoify(owoify_level), true)
            .color(Colour::from_rgb(242, 153, 169))
            .description(format!("type: {:?}", owoify_level))
        })
    })
    .await?;
    Ok(())
}
