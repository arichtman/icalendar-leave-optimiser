#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unreachable_code,
    unused_mut
)]

use chrono::NaiveDate;
use std::fs::File;
use std::io::prelude::*;

extern crate clap;
use clap::{arg, command, Parser};

use icalendar::{Calendar, CalendarComponent, Component, DatePerhapsTime, Event, EventLike};
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

    let mut demo_month = [
        false, false, false, false, false, true, true, false, false, false, false, false, true,
        true, false, false, false, false, false, true, true, false, false, false, false, false,
        true, true, false, false, false,
    ];
    fn calculate_skew(state: &[bool]) -> f32 {
        4.0
    }
    fn calculate_contiguous(state: &[bool]) -> f32 {
        2.0
    }
    fn calculate_fitness(skew: f32, sw: f32, contiguous_value: f32, cw: f32) -> u32 {
        // This is a rubish formula to illustrate the different ways of applying our aspects
        const FORM_CONST: u32 = 50;
        (contiguous_value.powf(cw) + (skew * sw) + FORM_CONST as f32) as u32
    }
    fn assess_fitness(state: &[bool]) -> i32 {
        let skew_value = calculate_skew(state);
        let contiguous_value = calculate_contiguous(state);
        const CONTIGUOUS_WEIGHT: f32 = 1.0;
        const SKEW_WEIGHT: f32 = 1.0;
        calculate_fitness(skew_value, SKEW_WEIGHT, contiguous_value, CONTIGUOUS_WEIGHT)
            .try_into()
            .unwrap()
    }
    fn select_next_state(state: &[bool]) -> Vec<bool> {
        let original_fitness_score = assess_fitness(&state);
        let next_allocation = select_next_allocation(&state);
        let mut next_state: Vec<bool> = state.into();
        next_state[next_allocation] = true;
        next_state.into()
    }
    fn select_next_allocation(state: &[bool]) -> usize {
        // Dumb implementation that's random
        let mut rng = rand::thread_rng();
        rng.gen_range(0..state.len())
    }
    let current_state = demo_month;
    for days_allocated in 0..5 {
        let current_state = select_next_state(&current_state);
    }
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

#[cfg(test)]
mod tests;
