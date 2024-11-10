use chrono::NaiveDate;
use icalendar::Component;
use icalendar::{CalendarDateTime, DatePerhapsTime, Event};

#[derive(Debug)]
pub(super) struct SimplifiedEvent {
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub summary: String,
}

// TODO: impl Error trait and add messages?
#[derive(Debug)]
pub enum EventConversionError {
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
        let (start, end) = match start > end {
            true => (end, start),
            false => (start, end),
        };
        Ok(SimplifiedEvent {
            start,
            end,
            summary: summary.to_string(),
        })
    }
}

pub trait AsNaiveDate {
    fn naive_date(self) -> NaiveDate;
}

impl AsNaiveDate for DatePerhapsTime {
    fn naive_date(self) -> NaiveDate {
        use CalendarDateTime::*;
        use DatePerhapsTime::*;
        match self {
            Date(date) => date,
            DateTime(Floating(date_time)) => date_time.date(),
            DateTime(CalendarDateTime::Utc(date_time)) => date_time.date_naive(),
            DateTime(WithTimezone { date_time, tzid: _ }) => date_time.date(),
        }
    }
}
