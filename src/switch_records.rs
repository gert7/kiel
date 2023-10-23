use crate::schema::switch_records;
use diesel::prelude::*;
use diesel::PgConnection;

use crate::strategy::PowerState;

#[derive(Insertable)]
#[table_name = "switch_records"]
pub struct NewSwitchRecord {
    state: i32,
}

fn switch_to_int(state: &PowerState) -> i8 {
    match state {
        PowerState::On => 1,
        PowerState::Off => 0,
    }
}

pub fn record_switch(connection: &PgConnection, power_state: &PowerState) -> eyre::Result<i32> {
    use crate::schema::switch_records::dsl::*;

    let nid: i32 = diesel::insert_into(switch_records)
        .values(NewSwitchRecord {
            state: switch_to_int(power_state).into(),
        })
        .returning(id)
        .get_result(connection)?;
    Ok(nid)
}
