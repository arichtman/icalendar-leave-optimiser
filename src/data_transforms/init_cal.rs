use chrono::{Days, NaiveDate};
#[derive(Debug)]
pub struct InitializedCalendar {
    pub dates: Vec<NaiveDate>,
}

impl InitializedCalendar {
    pub fn new(initial_date: NaiveDate, duration: u64) -> InitializedCalendar {
        let dates = (0..=duration)
            .map(|offset| initial_date.checked_add_days(Days::new(offset)).unwrap())
            .collect();
        InitializedCalendar { dates }
    }
}
