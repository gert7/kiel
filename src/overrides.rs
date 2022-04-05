// override is a reserved keyword

use chrono::{Timelike, Weekday, Datelike};
use chrono_tz::Tz;

use crate::{strategy::{PriceChangeUnit, PowerState}, config_file::{ConfigFile, Day}};

struct Oride {
    day: Weekday,
    hour: u32,
    state: PowerState
}

fn append_day(vec: &mut Vec<Oride>, day: &Day, weekday: Weekday) {
    if let Some(orides) = &day.hours_always_on {
        for &oride in orides {
            vec.push(Oride {
                day: weekday,
                hour: oride.into(),
                state: PowerState::On,
            })
        }
    }

    if let Some(orides) = &day.hours_always_off {
        for &oride in orides {
            vec.push(Oride {
                day: weekday,
                hour: oride.into(),
                state: PowerState::Off,
            })
        }
    }
}

fn get_overrides_from_config(config: &ConfigFile) -> Vec<Oride> {
    let mut vec = vec![];
    append_day(&mut vec, &config.monday, Weekday::Mon);
    append_day(&mut vec, &config.tuesday, Weekday::Tue);
    append_day(&mut vec, &config.wednesday, Weekday::Wed);
    append_day(&mut vec, &config.thursday, Weekday::Thu);
    append_day(&mut vec, &config.friday, Weekday::Fri);
    append_day(&mut vec, &config.saturday, Weekday::Sat);
    append_day(&mut vec, &config.sunday, Weekday::Sun);
    vec
}

pub fn apply_overrides(
    vec: &mut Vec<PriceChangeUnit>,
    config: &ConfigFile,
    timezone: &Tz
) {
    let orides = get_overrides_from_config(config);

    for pcu in vec.iter_mut() {
        let local_time = pcu.moment.with_timezone(timezone);
        let day = local_time.weekday();
        let hour = local_time.hour();
        let oride = orides
            .iter()
            .find(|&oride| oride.day == day && oride.hour == hour);
        if let Some(oride) = oride {
            (*pcu).state = oride.state;
        }
    }
}
