use chrono::Weekday;
use color_eyre::eyre::{self, eyre};
use serde::{Deserialize, Serialize};
use toml::Value;

use crate::strategy::{
    always::{AlwaysOffStrategy, AlwaysOnStrategy},
    default::TariffStrategy,
    limit::PriceLimitStrategy,
    smart::SmartStrategy,
};

#[derive(Deserialize)]
#[serde(tag = "mode")]
pub enum DayBasePlan {
    AlwaysOff(AlwaysOffStrategy),
    AlwaysOn(AlwaysOnStrategy),
    Tariff(TariffStrategy),
}

#[derive(Deserialize)]
#[serde(tag = "mode")]
pub enum DayStrategy {
    Limit(PriceLimitStrategy),
    Smart(SmartStrategy),
}

#[derive(Deserialize)]
pub struct Day {
    pub hours_always_on: Option<Vec<u32>>,
    pub hours_always_off: Option<Vec<u32>>,
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
    pub fn decode_config(filename: &str) -> eyre::Result<ConfigFile> {
        let conf = std::fs::read_to_string(filename)?;
        let config_file = toml::from_str::<ConfigFile>(&conf).unwrap();
        Ok(config_file)
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
}
