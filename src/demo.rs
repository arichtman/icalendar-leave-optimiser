use chrono::NaiveDate;
use icalendar::{Calendar, CalendarComponent, Component, DatePerhapsTime, Event, EventLike};
use std::fs::File;
use std::io::prelude::*;

pub fn create_and_write_demo_calendar() -> Result<(), std::io::Error> {
    /*
    Working backwards, we want a fully populated Calendar object, so we can serialize it to disk.
    Is this even possible?
    Let's construct one manually...
    */
    let mut test_output_cal = Calendar::new();
    let cal_event = Event::new()
        .status(icalendar::EventStatus::Confirmed)
        .starts(DatePerhapsTime::Date(
            NaiveDate::from_yo_opt(2024, 1).unwrap(),
        ))
        .ends(DatePerhapsTime::Date(
            NaiveDate::from_yo_opt(2024, 5).unwrap(),
        ))
        .summary("my event")
        .done();
    let cal_comp = CalendarComponent::Event(cal_event);
    test_output_cal.push(cal_comp);
    let mut file = File::create("test_output.ics")?;
    file.write_all(&format!("{test_output_cal}").into_bytes())?;
    Ok(())
}
