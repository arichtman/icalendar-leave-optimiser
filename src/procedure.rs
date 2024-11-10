use chrono::Datelike;
use icalendar::Calendar;
use log::{debug, info};
use std::fs;
use std::path::PathBuf;

// TODO: This importing smells, they shouldn't be submodules?
use crate::data_transforms::hol_cal::HolidayedCalendar;
use crate::data_transforms::init_cal::InitializedCalendar;
use chrono::NaiveDate;
use chrono::Utc;

pub fn do_procedure(
    input_file_paths: Vec<PathBuf>,
    duration: u64,
) -> Result<Calendar, std::io::Error> {
    let file_path = &input_file_paths[0];
    let file_contents = fs::read_to_string(file_path)?;
    // TODO: Check up error handling practices
    let cal = file_contents
        .parse::<Calendar>()
        .expect("Unable to parse calendar from file.");
    let now = Utc::now();
    let now = NaiveDate::from_num_days_from_ce_opt(now.num_days_from_ce()).unwrap();
    let initial_calendar = InitializedCalendar::new(now, duration);
    info!("{initial_calendar:?}");
    let holiday_calendar = HolidayedCalendar::new(initial_calendar, &cal);
    debug!("************************");
    debug!("{holiday_calendar:?}");
    todo!()
}
