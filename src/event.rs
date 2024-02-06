use poise::serenity_prelude as serenity;
use serenity::all::FullEvent;

use crate::data::{Data};
use crate::utils::{Error, discord::activity::global_presence};


pub async fn event_handler(
    ctx: &serenity::Context,
   event: &FullEvent,
   _framework: poise::FrameworkContext<'_, Data, Error>,
   _data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready {..} => {
            let message = String::from("Upgrading rn | 全体更新中");
            global_presence(ctx, message)
        }
        _ => {}
    }
    Ok(())
}
