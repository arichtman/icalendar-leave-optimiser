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
        let start = start.date_naive();
        let end = end.date_naive();
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
