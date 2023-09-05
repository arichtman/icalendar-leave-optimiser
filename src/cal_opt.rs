use clap::error::ContextKind;
use icalendar::DatePerhapsTime;

use icalendar::Event;
use icalendar::{
    parser, Calendar, CalendarComponent, CalendarDateTime, CalendarDateTime::*, Component,
};
use log::{debug, error, info, warn};
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::PathBuf;

use chrono::NaiveDate;

mod traits;
use crate::cal_opt::traits::AsNaiveDate;

pub fn cal_opt(input_file_paths: Vec<PathBuf>, duration: u32) -> Calendar {
    for file_path in input_file_paths {
        let file_contents = match fs::read_to_string(&file_path) {
            Ok(fc) => fc,
            Err(err) => {
                error!("Unable to read file {}, Skipping.", file_path.display());
                debug!("{}", err);
                continue;
            }
        };
        // TODO: Check up error handling practices
        let cal = file_contents
            .parse::<Calendar>()
            .expect("Unable to parse calendar from file.");
        // Filter to remove anything that's not an event
        // TODO: Rationalize this, triple-filtering seems... off
        let events = cal
            .iter()
            .filter(|x| matches!(x, CalendarComponent::Event(_)))
            .filter_map(|cc| cc.as_event())
            .collect::<Vec<_>>();
        info!("Found {} events total.", events.len());
        debug!("{:?}", events);
        for event in events {
            // TODO: I'd prefer more control here over failures.
            // I'm thinking default behaviour to continue with an option to make strict
            // I'd also like to be able to log at different levels on a skip
            let end_date = event
                .get_end()
                .expect(
                    format!(
                        "Unable to retrieve end for {:?}",
                        event.property_value("SUMMARY").unwrap()
                    )
                    .as_str(),
                )
                .naive_date();
            let start_date = event
                .get_start()
                .expect(
                    format!(
                        "Unable to retrieve start for {:?}",
                        event.property_value("SUMMARY").unwrap()
                    )
                    .as_str(),
                )
                .naive_date();
            info!(
                "{}: {:#?} - {:#?}",
                event
                    .get_summary()
                    .expect("Summary property missing for event."),
                start_date,
                end_date
            );
            debug!("{:?}", event)
        }
    }
    // This is just a placeholder to keep the ide quiet
    Calendar::default()
}
