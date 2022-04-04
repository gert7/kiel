use crate::schema::power_states;
use crate::{constants::PLANNING_TZ, price_cell::get_day_start_end};
use chrono::{Date, DateTime, Utc};
use chrono_tz::Tz;
use color_eyre::eyre;
use diesel::{prelude::*, PgConnection};

use super::{PowerState, PriceChangeUnit};

#[derive(Queryable)]
pub struct PowerStateDB {
    id: i32,
    moment_utc: DateTime<Utc>,
    state: i32,
    configuration_id: Option<i32>,
    created_at: DateTime<Utc>,
}

impl PowerStateDB {
    pub fn num_to_state(num: i32) -> PowerState {
        match num {
            0 => PowerState::Off,
            1 => PowerState::On,
            _ => PowerState::Off,
        }
    }

    pub fn state_to_num(num: PowerState) -> i32 {
        match num {
            PowerState::On => 1,
            PowerState::Off => 0,
        }
    }

    pub fn get_day_from_database<'a>(
        connection: &'a PgConnection,
        day: &'a Date<Tz>,
        configuration_id_val: Option<i32>,
    ) -> eyre::Result<Vec<PriceChangeUnit<'a>>> {
        use crate::schema::power_states::dsl::*;

        let (day_start, day_end) = get_day_start_end(&day);

        let result = power_states
            .filter(moment_utc.ge(&day_start))
            .filter(moment_utc.lt(&day_end))
            .filter(configuration_id.eq(configuration_id_val))
            .limit(48)
            .load::<PowerStateDB>(connection)?;

        let vec = result.into_iter().map(|psdb| psdb.into()).collect();

        Ok(vec)
    }
}

impl<'a> From<PowerStateDB> for PriceChangeUnit<'a> {
    fn from(psdb: PowerStateDB) -> Self {
        PriceChangeUnit {
            moment: psdb.moment_utc.with_timezone(&PLANNING_TZ),
            price: None,
            state: PowerStateDB::num_to_state(psdb.state),
        }
    }
}

#[derive(Insertable)]
#[table_name = "power_states"]
pub struct NewPowerStateDB {
    moment_utc: DateTime<Utc>,
    state: i32,
    configuration_id: Option<i32>,
}

impl NewPowerStateDB {
    fn from_pcu(pcu: PriceChangeUnit, configuration_id: Option<i32>) -> Self {
        NewPowerStateDB {
            moment_utc: pcu.moment.with_timezone(&Utc),
            state: PowerStateDB::state_to_num(pcu.state),
            configuration_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;
    use crate::{
        constants::{HOURS_OF_DAY, MARKET_TZ},
        database,
    };

    fn clear_table(connection: &PgConnection) {
        diesel::delete(power_states::table).execute(connection).ok();
    }

    fn day_sample(date: &Date<Tz>, states: Vec<PowerState>, cfid: i32) -> Vec<NewPowerStateDB> {
        let mut vec = vec![];
        for (hour, s) in states.into_iter().enumerate() {
            vec.push(NewPowerStateDB {
                moment_utc: date
                    .and_hms(hour.try_into().unwrap(), 0, 0)
                    .with_timezone(&Utc),
                state: PowerStateDB::state_to_num(s),
                configuration_id: Some(cfid),
            })
        }
        vec
    }

    fn day_sample_checkerboard(date: &Date<Tz>, cfid: i32) -> Vec<NewPowerStateDB> {
        let mut vec = vec![];
        for hour in HOURS_OF_DAY {
            let state = hour % 2;
            vec.push(NewPowerStateDB {
                moment_utc: date
                    .and_hms(hour.try_into().unwrap(), 0, 0)
                    .with_timezone(&Utc),
                state: state.try_into().unwrap(),
                configuration_id: Some(cfid),
            })
        }
        vec
    }

    fn insert_checkerboard(connection: &PgConnection, date: &Date<Tz>, cfid: i32) {
        let samples = day_sample_checkerboard(date, 71);
        diesel::insert_into(power_states::table)
            .values(&samples)
            .execute(connection)
            .expect("Unable to insert!");
    }

    #[test]
    fn fetch_from_database() {
        use crate::schema::power_states::dsl::*;
        let connection = database::establish_connection();
        clear_table(&connection);
        let day_date = MARKET_TZ.ymd(2022, 3, 13);
        insert_checkerboard(&connection, &day_date, 71);
        let day = PowerStateDB::get_day_from_database(&connection, &day_date, Some(71)).unwrap();
        for hour in HOURS_OF_DAY {
            let expected: i32 = (hour % 2).try_into().unwrap();
            let index: usize = hour.try_into().unwrap();
            let actual = PowerStateDB::state_to_num(day[index].state);
            assert!(expected == actual);
        }
    }
}
