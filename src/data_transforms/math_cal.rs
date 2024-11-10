use derive_more::derive::Display;

use super::hol_cal::HolidayedCalendar;

#[derive(Debug)]
pub struct MathCalendar {
    math_representation: Vec<u8>,
}

impl MathCalendar {
    pub fn new() -> Self {
        MathCalendar {
            math_representation: vec![],
        }
    }
}

impl From<HolidayedCalendar> for MathCalendar {
    fn from(value: HolidayedCalendar) -> Self {
        todo!()
    }
}
