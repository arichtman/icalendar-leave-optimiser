use std::{path::PathBuf, str::FromStr};

use crate::cal_opt;

#[test]
fn assertion_check() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let cal = cal_opt(vec![PathBuf::from_str("qld.ics").unwrap()], 365);
}
