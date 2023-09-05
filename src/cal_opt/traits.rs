use chrono::NaiveDate;
use icalendar::CalendarDateTime;
use icalendar::CalendarDateTime::Floating;
use icalendar::CalendarDateTime::WithTimezone;
use icalendar::DatePerhapsTime;
use icalendar::DatePerhapsTime::Date as iCalDate;
use icalendar::DatePerhapsTime::DateTime as iCalDateTime;

pub trait AsNaiveDate {
    fn naive_date(self) -> NaiveDate;
}

impl AsNaiveDate for DatePerhapsTime {
    fn naive_date(self) -> NaiveDate {
        match self {
            iCalDate(date) => date,
            iCalDateTime(Floating(date_time)) => date_time.date(),
            iCalDateTime(CalendarDateTime::Utc(date_time)) => date_time.date_naive(),
            iCalDateTime(WithTimezone { date_time, tzid: _ }) => date_time.date(),
        }
    }
}
