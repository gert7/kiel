#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Datelike, Utc};
    use color_eyre::eyre;
    use diesel::{prelude::*, PgConnection};
    use rand::thread_rng;

    use crate::{
        config_file::{tests::insert_good_cfg, ConfigFile, DayBasePlan},
        constants::{DEFAULT_CONFIG_FILENAME, MARKET_TZ, LOCAL_TZ, PLANNING_TZ},
        database::establish_connection,
        price_cell::PriceCell,
        sample_data,
        schema::{
            convar_ints, convar_strings, day_configurations, power_states, price_cells,
            switch_records,
        }, strategy::{default::TariffStrategy, PowerState, power_state_model::PowerStateDB}, overrides, planner_main,
    };

    fn clear_all_tables(connection: &PgConnection) -> eyre::Result<()> {
        diesel::delete(power_states::table).execute(connection)?;
        diesel::delete(day_configurations::table).execute(connection)?;
        diesel::delete(price_cells::table).execute(connection)?;
        diesel::delete(switch_records::table).execute(connection)?;
        diesel::delete(convar_ints::table).execute(connection)?;
        diesel::delete(convar_strings::table).execute(connection)?;
        Ok(())
    }

    #[test]
    fn integrate() {
        let connection = establish_connection();
        clear_all_tables(&connection).unwrap();
        let start_date = MARKET_TZ.ymd(2022, 3, 13); // Sunday
        let sample_day = sample_data::sample_day(&start_date, 0, 24, &mut thread_rng());
        PriceCell::insert_cells_into_database(&connection, &sample_day.0).unwrap();

        insert_good_cfg(&connection);

        let (cfdb, config) =
            ConfigFile::fetch_with_default(&connection, DEFAULT_CONFIG_FILENAME).unwrap();
        assert!(cfdb.is_some());
        let cfdb_id = cfdb.unwrap().id;

        let pdb = PriceCell::get_prices_from_db(&connection, &start_date);
        let config_today = config.get_day(&start_date.weekday());

        let base = config_today.base.unwrap_or(DayBasePlan::Tariff(TariffStrategy));
        let base_prices = base.get_hour_strategy().plan_day_full(&pdb, &start_date);

        let mut strategy_result = match config_today.strategy {
            Some(strategy) => strategy.get_day_strategy().plan_day_masked(&base_prices),
            None => base_prices,
        };

        overrides::apply_overrides(&mut strategy_result, &config, &LOCAL_TZ);

        PowerStateDB::insert_day_into_database(&connection, &strategy_result, Some(cfdb_id));

        for h in 0..=13 {
            println!("{}", h);
            assert!(strategy_result[h].state == PowerState::On);
        }

        for h in 14..=15 {
            assert!(strategy_result[h].state == PowerState::Off);
        }

        for h in 16..=23 {
            assert!(strategy_result[h].state == PowerState::On);
        }

        let mut cached_states = PowerStateDB::get_day_from_database(&connection, &start_date, Some(cfdb_id)).unwrap();

        println!("{:?}", cached_states);

        cached_states.sort_by(|a, b| a.moment.cmp(&b.moment));

        for h in 0..=13 {
            println!("{}", h);
            assert!(cached_states[h].state == PowerState::On);
        }

        for h in 14..=15 {
            assert!(cached_states[h].state == PowerState::Off);
        }

        for h in 16..=23 {
            assert!(cached_states[h].state == PowerState::On);
        }

        println!("{:?}", strategy_result);
    }

    #[tokio::test]
    async fn integrate_tomorrow() {
        let connection = establish_connection();
        clear_all_tables(&connection).unwrap();
        let start_date = MARKET_TZ.ymd(2022, 3, 13); // Sunday
        let sample_day = sample_data::sample_day(&start_date, 0, 24, &mut thread_rng());
        PriceCell::insert_cells_into_database(&connection, &sample_day.0).unwrap();
        let now = Utc::now().with_timezone(&PLANNING_TZ);
        planner_main(true, now).await.unwrap();
    }
}
