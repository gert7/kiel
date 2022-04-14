use chrono::{DateTime, Utc, Weekday};
use color_eyre::eyre::{self, eyre};
use diesel::{prelude::*, update, PgConnection};
use serde::{Deserialize, Serialize};

use crate::{
    constants::{CVAR_CONFIG_FAILURE_COUNT, DEFAULT_CONFIG_FILENAME},
    convars::{ConvarInt, NewConvarInt},
    schema::day_configurations,
    strategy::{
        always::{AlwaysOffStrategy, AlwaysOnStrategy},
        default::TariffStrategy,
        limit::PriceLimitStrategy,
        none::NoneStrategy,
        smart::SmartStrategy,
        HourStrategy, MaskablePowerStrategy,
    },
};

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(tag = "mode")]
pub enum DayBasePlan {
    AlwaysOff(AlwaysOffStrategy),
    AlwaysOn(AlwaysOnStrategy),
    Tariff(TariffStrategy),
}

impl DayBasePlan {
    pub fn get_hour_strategy(self) -> Box<dyn HourStrategy> {
        match self {
            DayBasePlan::AlwaysOff(v) => Box::new(v),
            DayBasePlan::AlwaysOn(v) => Box::new(v),
            DayBasePlan::Tariff(v) => Box::new(v),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(tag = "mode")]
pub enum DayStrategy {
    None(NoneStrategy),
    Limit(PriceLimitStrategy),
    Smart(SmartStrategy),
}

impl DayStrategy {
    pub fn get_day_strategy(self) -> Box<dyn MaskablePowerStrategy> {
        match self {
            DayStrategy::None(v) => Box::new(v),
            DayStrategy::Limit(v) => Box::new(v),
            DayStrategy::Smart(v) => Box::new(v),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Day {
    pub hours_always_on: Option<Vec<u8>>,
    pub hours_always_off: Option<Vec<u8>>,
    pub base: Option<DayBasePlan>,
    pub strategy: Option<DayStrategy>,
}

#[derive(Deserialize)]
pub struct ConfigFile {
    pub monday: Day,
    pub tuesday: Day,
    pub wednesday: Day,
    pub thursday: Day,
    pub friday: Day,
    pub saturday: Day,
    pub sunday: Day,
}

impl ConfigFile {
    pub fn decode_config(file: &str) -> eyre::Result<ConfigFile> {
        println!("{}", file);
        let config_file = toml::from_str::<ConfigFile>(file)?;
        Ok(config_file)
    }

    pub fn decode_file(filename: &str) -> eyre::Result<ConfigFile> {
        let conf = std::fs::read_to_string(filename)?;
        Ok(ConfigFile::decode_config(&conf)?)
    }

    pub fn get_day(&self, weekday: &Weekday) -> &Day {
        match weekday {
            Weekday::Mon => &self.monday,
            Weekday::Tue => &self.tuesday,
            Weekday::Wed => &self.wednesday,
            Weekday::Thu => &self.thursday,
            Weekday::Fri => &self.friday,
            Weekday::Sat => &self.saturday,
            Weekday::Sun => &self.sunday,
        }
    }

    pub fn last_id(connection: &PgConnection) -> Result<i32, diesel::result::Error> {
        use crate::schema::day_configurations::dsl::*;
        let row = day_configurations
            .order(id.desc())
            .first::<ConfigFileDB>(connection)?;
        Ok(row.id)
    }

    pub fn increment_failures(connection: &PgConnection) {
        use crate::schema::convar_ints::dsl::*;

        let failure_count = convar_ints
            .filter(key.eq(CVAR_CONFIG_FAILURE_COUNT))
            .order(id.desc())
            .first::<ConvarInt>(connection);
        let failure_count = failure_count.map_or(0, |fc| fc.value);
        let failures = NewConvarInt {
            key: CVAR_CONFIG_FAILURE_COUNT,
            value: failure_count + 1,
        };
        diesel::insert_into(convar_ints)
            .values(failures)
            .execute(connection)
            .ok();
    }

    pub fn reset_failures(connection: &PgConnection) {
        use crate::schema::convar_ints::dsl::*;

        let failures = NewConvarInt {
            key: CVAR_CONFIG_FAILURE_COUNT,
            value: 0,
        };
        diesel::insert_into(convar_ints)
            .values(failures)
            .execute(connection)
            .ok();
    }

    fn config_attempt_loop<'a>(
        connection: &PgConnection,
        cfgs: Vec<ConfigFileDB>,
    ) -> eyre::Result<(ConfigFileDB, ConfigFile)> {
        use crate::schema::day_configurations::dsl::*;
        for cfdb in cfgs {
            let attempt = Self::decode_config(&cfdb.toml);
            println!("attempt");
            match attempt {
                Ok(good) => {
                    if !cfdb.tried {
                        let db_result = update(day_configurations.filter(id.eq(cfdb.id)))
                            .set(tried.eq(true))
                            .execute(connection);
                        if let Err(e) = db_result {
                            eprintln!("{}", e);
                        };
                    }
                    Self::reset_failures(connection);
                    return Ok((cfdb, good));
                }
                Err(e) => {
                    eprintln!("{}", e);
                    let db_result = update(day_configurations.filter(id.eq(cfdb.id)))
                        .set((known_broken.eq(true), tried.eq(true)))
                        .execute(connection);
                    Self::increment_failures(connection);
                    if let Err(e) = db_result {
                        eprintln!("{}", e);
                    };
                }
            }
        }
        Err(eyre!("No good config found"))
    }

    pub fn fetch_from_database(
        connection: &PgConnection,
    ) -> eyre::Result<(ConfigFileDB, ConfigFile)> {
        use crate::schema::day_configurations::dsl::*;

        let find = day_configurations
            .filter(known_broken.eq(false))
            .order(id.desc())
            .limit(10)
            .load::<ConfigFileDB>(connection)
            .expect("Unable to load configuration file from database");
        let cfg_pair = ConfigFile::config_attempt_loop(&connection, find)?;
        Ok(cfg_pair)
    }

    pub fn fetch_with_default(
        connection: &PgConnection,
        default_filename: &str,
    ) -> eyre::Result<(Option<ConfigFileDB>, ConfigFile)> {
        let result = ConfigFile::fetch_from_database(connection);
        if result.is_ok() {
            println!("Found configuration in database");
        }
        match result {
            Ok(cf) => Ok((Some(cf.0), cf.1)),
            Err(_) => Ok((
                None,
                ConfigFile::decode_file(default_filename).expect("No configuration file found!"),
            )),
        }
    }

    pub fn insert_string(connection: &PgConnection, file_string: &str) -> eyre::Result<i32> {
        use crate::schema::day_configurations::dsl::*;
        let conf_id: Vec<i32> = diesel::insert_into(day_configurations)
            .values(NewConfigFileDB {
                toml: file_string,
                known_broken: false,
                tried: false,
            })
            .returning(id)
            .get_results(connection)?;
        Ok(*conf_id
            .get(0)
            .expect("Unable to insert default configuration into database"))
    }

    pub fn fetch_with_default_inserting(
        connection: &PgConnection,
        default_filename: &str,
    ) -> eyre::Result<(i32, ConfigFile)> {
        let result = ConfigFile::fetch_from_database(connection);
        if result.is_ok() {
            println!("Found configuration in database");
        }
        match result {
            Ok(cf) => Ok((cf.0.id, cf.1)),
            Err(_) => {
                let toml = std::fs::read_to_string(default_filename)?;
                let id = ConfigFile::insert_string(connection, &toml)?;
                Ok((
                    id,
                    ConfigFile::decode_file(default_filename)
                        .expect("No configuration file found!"),
                ))
            }
        }
    }
}

#[derive(Queryable)]
pub struct ConfigFileDB {
    pub id: i32,
    pub toml: String,
    pub known_broken: bool,
    pub tried: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "day_configurations"]
struct NewConfigFileDB<'a> {
    toml: &'a str,
    known_broken: bool,
    tried: bool,
}

#[cfg(test)]
pub mod tests {
    use core::num;

    use crate::database;

    use super::*;
    use crate::schema::day_configurations::dsl::*;
    use diesel::prelude::*;

    fn clear_table(connection: &PgConnection) {
        diesel::delete(day_configurations).execute(connection).ok();
    }

    pub fn insert_good_cfg(connection: &PgConnection) -> ConfigFileDB {
        let good_toml = std::fs::read_to_string("samples/default.toml").unwrap();
        let new_cfg = NewConfigFileDB {
            toml: &good_toml,
            known_broken: false,
            tried: false,
        };
        diesel::insert_into(day_configurations)
            .values(new_cfg)
            .get_result(connection)
            .expect("Unable to insert!")
    }

    const BAD_TOML: &str = "jwraiojfoad";

    fn insert_bad_cfg(connection: &PgConnection, known_broken_val: bool) -> ConfigFileDB {
        let new_cfg = NewConfigFileDB {
            toml: BAD_TOML,
            known_broken: known_broken_val,
            tried: false,
        };
        diesel::insert_into(day_configurations)
            .values(new_cfg)
            .get_result(connection)
            .expect("Unable to insert!")
    }

    #[test]
    fn loads_from_database() {
        let connection = database::establish_connection();
        clear_table(&connection);
        insert_good_cfg(&connection);
        let loaded = ConfigFile::fetch_with_default(&connection, DEFAULT_CONFIG_FILENAME);
        assert!(loaded.is_ok());
    }

    #[test]
    fn loads_default_with_empty_database() {
        let connection = database::establish_connection();
        clear_table(&connection);
        let loaded = ConfigFile::fetch_with_default(&connection, DEFAULT_CONFIG_FILENAME);
        assert!(loaded.is_ok());
    }

    #[test]
    #[should_panic]
    fn fails_with_wrong_default_config() {
        let connection = database::establish_connection();
        clear_table(&connection);
        ConfigFile::fetch_with_default(&connection, "samples/fjafiowje.toml").ok();
    }

    #[test]
    fn marks_broken_configs_correctly() {
        let connection = database::establish_connection();
        clear_table(&connection);
        let db_good = insert_good_cfg(&connection);
        let db_bad = insert_bad_cfg(&connection, false);
        let good = ConfigFile::fetch_with_default(&connection, DEFAULT_CONFIG_FILENAME);
        assert!(good.is_ok());

        let db_good: ConfigFileDB = day_configurations
            .find(db_good.id)
            .first(&connection)
            .unwrap();
        let db_bad: ConfigFileDB = day_configurations
            .find(db_bad.id)
            .first(&connection)
            .unwrap();
        assert!(db_good.known_broken == false);
        assert!(db_bad.known_broken == true);
    }
}
