use std::env;

use color_eyre::eyre;
use diesel::PgConnection;

use crate::{strategy::PowerState, switch_records::record_switch};

pub async fn apply_power_state(connection: &PgConnection, state: &PowerState) -> eyre::Result<()> {
    let post_url = match state {
        PowerState::On => env::var("WEBHOOK_POST_ON")?,
        PowerState::Off => env::var("WEBHOOK_POST_OFF")?,
    };
    match state {
        PowerState::On => println!("Turned on!"),
        PowerState::Off => println!("Turned off!"),
    };
    let client = reqwest::Client::new();
    client.post(post_url).send().await?;
    record_switch(connection, state)?;
    Ok(())
}
