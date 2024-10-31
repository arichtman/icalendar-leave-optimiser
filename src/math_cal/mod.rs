use core::panic;

use chrono::NaiveDate;
use icalendar::Calendar;
use log::{debug, error, info, warn};

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
