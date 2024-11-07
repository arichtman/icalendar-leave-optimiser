use chrono::Days;
use core::panic;
use log::debug;

use chrono::NaiveDate;
use icalendar::{Calendar, CalendarDateTime, DatePerhapsTime, Event};
use icalendar::{CalendarComponent, Component};

use crate::cal_opt::AsNaiveDate;

struct InterimCalendar {
    dates: Vec<(NaiveDate, bool)>,
}

impl InterimCalendar {
    fn new(initial_date: NaiveDate, duration: u64) -> InterimCalendar {
        let dates = (0..=duration)
            .into_iter()
            .enumerate()
            .map(|(index, offset)| {
                (
                    initial_date.checked_add_days(Days::new(offset)).unwrap(),
                    false,
                )
            })
            .collect();
        InterimCalendar { dates }
    }
    fn populate_holidays(self: &mut Self, cal: &Calendar) {
        // Pull Event references out from Calendar
        let input_events = cal
            .components
            .iter()
            .map(|cc| cc.as_event())
            .collect::<Vec<_>>();
        debug!("{input_events:?}");
        // for input_event in input_events.iter().filter(|e| e.is_some()) {
        //     InterimCalendar::test_event_validity(input_event.unwrap());
        // }
        let valid_events: Vec<_> = input_events
            .iter()
            .filter(|e| InterimCalendar::test_event_validity(**e))
            .map(|e| e.unwrap())
            .collect();
        debug!("{valid_events:?}");
        for event in valid_events {
            event.get_start().unwrap().naive_date() >= event.get_end().unwrap().naive_date();
        }
        ()
    }
    fn test_event_validity(untested_event: Option<&Event>) -> bool {
        // Bail if None
        let Some(existing_event) = untested_event else {
            return false;
        };
        // Require at least summary text
        let summary = existing_event.get_summary().unwrap_or_default();
        if summary.is_empty() {
            return false;
        }

        // Chronological checks
        let start = existing_event.get_start();
        let end = existing_event.get_end();
        // Need both start and end to process
        if start.is_none() | end.is_none() {
            return false;
        }

        // Remove time granularity
        let start = start.unwrap().naive_date();
        let end = end.unwrap().naive_date();
        // On the odd chance the event is reversed
        if start > end {
            let (start, end) = (end, start);
        }
        true
    }
}

#[derive(Clone, Debug)]
pub struct MathDate {
    is_leave: bool,
    is_fixed: bool,
}

impl MathDate {
    fn new(is_leave: bool, is_fixed: bool) -> MathDate {
        MathDate { is_leave, is_fixed }
    }
}

#[derive(Debug)]
pub struct MathCalendar {
    days_store: Vec<MathDate>,
}

impl MathCalendar {
    fn new(days: u64) -> MathCalendar {
        let mut days_store = Vec::new();
        // TODO: see why total_days doesn't satisfy usize expected
        days_store.resize(days.try_into().unwrap(), MathDate::new(false, false));
        MathCalendar { days_store }
    }
}

/*
You can't really _convert_ an iCalendar into the thing we want. The additional parameters are missing.
You could add them to the calendar properties but really we need a wrapping type so they can be stored properly.

I'd start coding this but I think it's still too dependent on the actual thing I need to math on.
*/

impl From<Calendar> for MathCalendar {
    fn from(value: Calendar) -> Self {
        // For now, we'll use some fixed nonsense
        // TODO: This doesn't belong in this conversion, and we need to either bubble the error up
        //         or panic, or handle it, or shift this check into the Clap parameters.
        let start_date = NaiveDate::from_yo_opt(2024, 1).unwrap();
        let end_date = NaiveDate::from_yo_opt(2024, 366).unwrap();
        let total_days = (end_date - start_date).num_days();
        if total_days <= 0 {
            panic!("Invalid duration (zero or negative)")
        };
        let total_days = total_days.unsigned_abs();

        MathCalendar::new(total_days)
    }
}

pub fn make_math_calendar(cal: &Calendar) -> Result<MathCalendar, String> {
    Ok(Calendar::new().into())
}
