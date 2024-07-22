use std::process::Stdio;

use poise::{
    send_reply,
    serenity_prelude::{content_safe, Colour, ContentSafeOptions, CreateEmbed, CreateEmbedAuthor,Message, MessageBuilder},
    CreateReply,
};
use songbird::serenity;
use crate::{Context, Error};
use tokio::{io::AsyncWriteExt, process::Command};
#[poise::command(context_menu_command = "Check python code", track_edits)]
pub async fn check_code(
    ctx: Context<'_>,
    #[description = "Code to check"] msg: Message,
) -> Result<(), Error> {
    let firstchar = msg.content.find("```").expect("cannot find first");
    let firstline = msg.content[firstchar..].find('\n').expect("cannot find nl");
    let secondchar = msg.content[firstline..].find("```").expect("cannot find last");
    let st = &msg.content[firstline..firstline+secondchar];
    
    

    let mut checks = Command::new("ruff").args(["format","-"]).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("sas");
    checks.stdin.take().unwrap().write_all(st.as_bytes()).await;
    let out = checks.wait_with_output().await.unwrap();
    let formatted_code = String::from_utf8_lossy(&out.stdout);
    println!("format {}",formatted_code);
    // ctx.say(formatted_code).await;
    ctx.say(MessageBuilder::new().push_codeblock_safe(formatted_code, Some("Python")).build()).await;


    let mut checks = Command::new("ruff").args(["check","-"]).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("sas");
    checks.stdin.take().unwrap().write_all(st.as_bytes()).await;
    let out = checks.wait_with_output().await.unwrap();
    let a = String::from_utf8_lossy(&out.stdout);
    ctx.say(MessageBuilder::new().push_codeblock_safe(a, Some("bash")).build()).await;

    Ok(())
}