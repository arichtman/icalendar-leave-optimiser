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

#[derive(Debug)]
struct SimplifiedEvent {
    start: NaiveDate,
    end: NaiveDate,
    summary: String,
}

impl From<Event> for SimplifiedEvent {
    fn from(value: Event) -> Self {
        todo!()
    }
}

// TODO: impl Error trait and add messages?
#[derive(Debug)]
enum EventConversionError {
    MissingSummary,
    MissingBoundary,
}

// Q: Is this bad practice? Doing type conversion from reference to owned.
impl TryFrom<&Event> for SimplifiedEvent {
    type Error = EventConversionError;
    fn try_from(value: &Event) -> Result<Self, Self::Error> {
        // Require at least summary text
        let summary = value.get_summary().unwrap_or_default();
        if summary.is_empty() {
            return Err(EventConversionError::MissingSummary);
        }
        // Chronological checks need both start and end to process
        let (Some(start), Some(end)) = (value.get_start(), value.get_end()) else {
            return Err(EventConversionError::MissingBoundary);
        };

        // Remove time granularity
        let start = start.naive_date();
        let end = end.naive_date();
        // On the odd chance the event is reversed
        if start > end {
            let (start, end) = (end, start);
        }
        Ok(SimplifiedEvent {
            start,
            end,
            summary: summary.to_string(),
        })
    }
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
        // Pull references out from Calendar, drop non-Events and any unsuitable events
        let events = cal
            .components
            .iter()
            // Gather events only
            .filter_map(|cc| cc.as_event())
            // Convert to simplified events and drop any errors
            // Q: swallowing errors here smells, what ought I be doing?
            .filter_map(|e| SimplifiedEvent::try_from(e).ok())
            .collect::<Vec<_>>();
        let input_events = cal
            .components
            .iter()
            .filter_map(|cc| cc.as_event())
            // Q: What's the correct error handling here?
            .filter_map(|e| SimplifiedEvent::try_from(e).ok())
            .collect::<Vec<_>>();
        debug!("{input_events:?}");
        for event in input_events {
            ()
        }
        ()
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
