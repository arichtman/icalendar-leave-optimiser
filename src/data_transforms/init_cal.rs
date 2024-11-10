use chrono::{Days, NaiveDate};
use icalendar::Calendar;
#[derive(Debug)]
pub struct InitializedCalendar {
    dates: Vec<NaiveDate>,
    holiday_calendars: Vec<Calendar>,
}

impl InitializedCalendar {
    pub fn new(
        initial_date: NaiveDate,
        duration: u64,
        holiday_calendars: Vec<Calendar>,
    ) -> InitializedCalendar {
        let dates = (0..=duration)
            .map(|offset| initial_date.checked_add_days(Days::new(offset)).unwrap())
            .collect();
        InitializedCalendar {
            dates,
            holiday_calendars,
        }
    }

    // Q: When would you clone in here? Does it make sense to return the reference?
    pub fn get_dates(self: &Self) -> &Vec<NaiveDate> {
        &self.dates
    }
    pub fn get_holiday_calendars(self: &Self) -> &Vec<Calendar> {
        &self.holiday_calendars
    }
}
