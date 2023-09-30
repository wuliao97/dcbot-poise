use poise::serenity_prelude as serenity;
use poise::Event;

use crate::data::{Data};
use crate::utils::{Error, discord::activity::global_presence};


pub async fn event_handler(
    ctx: &serenity::Context,
   event: &Event<'_>,
   _framework: poise::FrameworkContext<'_, Data, Error>,
   _data: &Data,
) -> Result<(), Error> {
    match event {
        Event::Ready {..} => {
            // let x = &ctx.http.token;
            // dbg!(x);
            let message = String::from("Upgrading rn | 全体更新中");
            global_presence(ctx, message).await
        }
        _ => {}
    }
    Ok(())
}
