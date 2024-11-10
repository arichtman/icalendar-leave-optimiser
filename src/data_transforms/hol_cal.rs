use super::init_cal::InitializedCalendar;
use super::simp_event::SimplifiedEvent;
use crate::Calendar;
use chrono::NaiveDate;
use icalendar::CalendarComponent;
use log::debug;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HolidayedCalendar {
    dates: HashMap<NaiveDate, (String, bool)>,
}

impl From<InitializedCalendar> for HolidayedCalendar {
    fn from(value: InitializedCalendar) -> Self {
        let mut dates = value
            .get_dates()
            .clone()
            .into_iter()
            .map(|dt| (dt, ("".to_string(), false)))
            .collect::<HashMap<_, _>>();
        // Pull references out from Calendar, drop non-Events and any unsuitable events
        // TODO: this is terribly messy looking...
        let input_events = value
            .get_holiday_calendars()
            .into_iter()
            .fold::<Vec<CalendarComponent>, _>(vec![], |a, cc| {
                // Q: there must be a simpler way to non-mutably combine vectors
                let mut o = vec![];
                o.extend(a);
                o.extend(cc.components.clone());
                o
            })
            .iter()
            .filter_map(|cc| cc.as_event())
            .filter_map(|e| SimplifiedEvent::try_from(e).ok())
            .collect::<Vec<_>>();
        debug!("{input_events:?}");
        for event in input_events {
            debug!("{event:?}");
            // We know the start and end should be in order from the earlier conversion
            //   which avoids negatives.
            // However this cast would be lossy if >2^32 on 32-bit system etc
            //   but that's also way outside any sane operating bounds.
            //   but also...
            // Q: how could this be safer?
            let day_duration = (event.end - event.start).num_days() as usize;
            for date in event.start.iter_days().take(day_duration) {
                debug!("{date:?}");
                if let std::collections::hash_map::Entry::Occupied(mut e) = dates.entry(date) {
                    e.insert((event.summary.clone(), true));
                };
            }
        }
        debug!("{dates:?}");
        HolidayedCalendar { dates }
    }
}
// impl HolidayedCalendar {
//     // Q: What determines if my function should take references or ownership? Lifetime?
//     pub fn new(cal: InitializedCalendar) -> Self {
//     }
// }
