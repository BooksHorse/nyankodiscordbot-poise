//use anyhow::Error;
use chrono::{Datelike, Local, TimeZone, Utc};
use poise::{send_reply, serenity_prelude::MessageBuilder};

use crate::{Context, Error};
///check schedule
#[poise::command(slash_command)]
pub async fn subject(
    ctx: Context<'_>,
    #[description = "Room without slashes (312)"]
    #[min = 101]
    #[max = 614]
    room: Option<u16>

) -> Result<(), Error> {
    let sub = subjectlib::subject(Utc::now(), room.unwrap_or(312)).await;
    //Local::now().time() //mocktime :NaiveTime::from_hms(10,20,0)
    match sub {
        Ok(sub) => {
            send_reply(ctx, |f| {
                f.content(
                    MessageBuilder::new()
                        .push_safe(format!(
                            "{} at <t:{}:T>\n",
                            Local::now().weekday(),
                            Local::now().timestamp(),
                        ))
                        .push_codeblock_safe(sub.timetable_ascii, Some("markdown"))
                        .push({
                            match sub.current_subject {
                                Some(e) => format!(
                                    "{} ends in: <t:{}:R>",
                                    &e.name,
                                    Utc::now()
                                        .timestamp()
                                ),
                                None => "".to_owned(),
                            }
                        })
                        .build(),
                )
            })
            .await
            .unwrap();
            Ok(())
        }
        Err(why) => {
            send_reply(ctx, |f| f.content(format!("Err : {}", why))).await?;
            Ok(())
        }
    }
}
