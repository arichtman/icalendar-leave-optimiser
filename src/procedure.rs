use chrono::Datelike;
use icalendar::Calendar;
use log::{debug, info};
use std::path::PathBuf;

// TODO: This importing smells, they shouldn't be submodules?
use crate::data_transforms::hol_cal::HolidayedCalendar;
use crate::data_transforms::init_cal::InitializedCalendar;
use chrono::NaiveDate;
use chrono::Utc;

#[derive(Debug, thiserror::Error)]
pub enum CalOptProcessError {
    #[error("Something went wrong in-process")]
    GenericError,
}
pub fn do_procedure(
    holiday_input_calendar: Calendar,
    duration: u64,
) -> Result<Calendar, CalOptProcessError> {
    let now = Utc::now();
    let now = NaiveDate::from_num_days_from_ce_opt(now.num_days_from_ce()).unwrap();
    let initial_calendar = InitializedCalendar::new(now, duration);
    info!("{initial_calendar:?}");
    let holiday_calendar = HolidayedCalendar::new(initial_calendar, &holiday_input_calendar);
    debug!("************************");
    debug!("{holiday_calendar:?}");
    todo!()
}
