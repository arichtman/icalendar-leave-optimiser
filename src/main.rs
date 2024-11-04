#![allow(dead_code, unused_imports, unused_variables, unreachable_code)]

use chrono::NaiveDate;
use std::fs::File;
use std::io::prelude::*;

extern crate clap;
use clap::{arg, command, Parser};

use icalendar::{Calendar, CalendarComponent, Component, DatePerhapsTime, Event};
use log::{debug, info};

use std::path::PathBuf;

mod cal_opt;
mod math_cal;
use self::math_cal::*;
use crate::cal_opt::cal_opt;

use rand::Rng;

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let log_level = match cli.verbose {
        0 => log::Level::Error,
        1 => log::Level::Warn,
        2 => log::Level::Info,
        _ => log::Level::Debug,
    };
    simple_logger::init_with_level(log_level).expect("Error initialising logging, aborting.");

    debug!("{cli:#?}");
    let cal_opt = cal_opt(cli.file, cli.duration)?;
    debug!("{cal_opt:#?}");
    let _ = make_math_calendar(&cal_opt);

    // Testing using a mutable reference slice to avoid heap allocation with vec
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(1..10);
    let mut static_array = [0 as u8; 20];
    let array_ref = &mut static_array[..=length];
    array_ref[1] = 5;
    info!("{:?}", array_ref);
    info!("{:?}", &static_array[..=10]);
    create_and_write_demo_calendar()?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "icalendar-leave-optimiser", author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    /// Increments logging verbosity. May be applied multiple times.
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    /// iCalendar files containing existing holidays. May be specified multiple times.
    #[arg(short, long, action = clap::ArgAction::Append)]
    file: Vec<PathBuf>,
    /// Duration to plan for, in days, from today, exclusive.
    #[arg(short, long, action)]
    duration: u32,
}

#[cfg(test)]
mod tests;
fn create_and_write_demo_calendar() -> Result<(), std::io::Error> {
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
