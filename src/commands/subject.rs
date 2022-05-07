//use anyhow::Error;
use chrono::{Datelike, FixedOffset, Local, NaiveDateTime, SubsecRound, TimeZone};
use poise::{send_reply, serenity_prelude::MessageBuilder};

use crate::{Context, Error};
///check schedule
#[poise::command(slash_command)]
pub async fn subject(
    ctx: Context<'_>,
    #[description = "Room without slashes (312)"]
    #[min = 101]
    #[max = 614]
    room: Option<u16>,
) -> Result<(), Error> {
    let sub = subjectlib::subject(Local::now().time(), room.unwrap_or(312)).await;
    //Local::now().time() //mocktime :NaiveTime::from_hms(10,20,0)
    match sub {
        Ok(sub) => {
            send_reply(ctx, |f| {
                f.content(format!(
                    "{} at {}\n{}",
                    Local::now().weekday(),
                    Local::now().time(),
                    MessageBuilder::new()
                        .push_safe(format!(
                            "{} at {}\n",
                            Local::now().weekday(),
                            Local::now().time().round_subsecs(0),
                        ))
                        .push_codeblock_safe(sub.timetable_ascii, Some("markdown"),)
                        .push({
                            match sub.current_subject {
                                Some(e) => format!(
                                    "{} ends in: <t:{}:R>",
                                    &e.name,
                                    FixedOffset::east(7 * 3600)
                                        .from_local_datetime(&NaiveDateTime::new(
                                            Local::now().date().naive_local(),
                                            e.time_end
                                        ))
                                        .unwrap()
                                        .timestamp()
                                ),
                                None => "".to_string(),
                            }
                        })
                        .build(),
                ))
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
