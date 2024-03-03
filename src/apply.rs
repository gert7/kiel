use std::{env, process::Command};

use diesel::PgConnection;

use crate::{strategy::PowerState, switch_records::record_switch};

pub fn apply_power_state(connection: &PgConnection, state: &PowerState) -> eyre::Result<()> {
    let post_url = match state {
        PowerState::On => env::var("WEBHOOK_POST_ON")?,
        PowerState::Off => env::var("WEBHOOK_POST_OFF")?,
    };
    let mode = env::var("SWITCH_MODE")?;
    if mode == "HASS" {
        ureq::post(&post_url).call()?;
    } else if mode == "DIRECT" {
        let state = match state {
            PowerState::On => "on",
            PowerState::Off => "off",
        };
        Command::new("python3")
            .arg("/usr/local/bin/kieldirect")
            .arg(state)
            .output()
            .expect("Failed to execute direct command!");
    }
    record_switch(connection, state)?;
    match state {
        PowerState::On => println!("Turned on!"),
        PowerState::Off => println!("Turned off!"),
    };
    Ok(())
}
