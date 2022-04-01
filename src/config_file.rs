use color_eyre::eyre::{self, eyre};
use serde::{Deserialize, Serialize};
use toml::Value;

use crate::strategy::{smart::SmartStrategy, always::{AlwaysOnStrategy, AlwaysOffStrategy}, limit::PriceLimitStrategy};

#[derive(Deserialize)]
#[serde(tag = "mode")]
pub enum DayStrategyConfig {
    AlwaysOff(AlwaysOffStrategy),
    AlwaysOn(AlwaysOnStrategy),
    Smart(SmartStrategy),
}

#[derive(Deserialize)]
#[serde(tag = "mode")]
pub enum DayStrategyFilter {
    PriceLimit(PriceLimitStrategy)
}

#[derive(Deserialize)]
pub struct Day {
    pub hours_always_on: Option<Vec<u32>>,
    pub hours_always_off: Option<Vec<u32>>,
    pub config: Option<DayStrategyConfig>,
    pub filter: Option<DayStrategyFilter>,
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

pub fn decode_config() -> eyre::Result<()> {
    let conf = std::fs::read_to_string("asdf.toml")?;
    let config_file = toml::from_str::<ConfigFile>(&conf).unwrap();
    let smart = config_file.tuesday.config.unwrap();
    match smart {
        DayStrategyConfig::AlwaysOn(cfg) => {
            println!("Always on");
        },
        DayStrategyConfig::Smart(cfg) => todo!(),
        DayStrategyConfig::AlwaysOff(_) => todo!(),
    }
    Ok(())
}
