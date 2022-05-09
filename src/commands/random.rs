use poise::send_reply;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::{Context, Error};
/// Random number
#[poise::command(slash_command, prefix_command, reuse_response, guild_only)]
pub async fn random(
    ctx: Context<'_>,
    #[description = "Minimum"]
    #[min = -2147483648]
    min: Option<i32>,
    #[description = "Maximum"]
    #[max = 2147483647]
    max: Option<i32>,
) -> Result<(), Error> {
    let min = min.unwrap_or(0);
    let max = max.unwrap_or(10);

    if min == max {
        send_reply(ctx, |f| f.content(min.to_string())).await?;
    }
    //let mut rng = rand::thread_rng();
    let mut rng = ChaCha20Rng::from_entropy();

    send_reply(ctx, |f| f.content(rng.gen_range(min..=max).to_string())).await?;
    Ok(())
}
