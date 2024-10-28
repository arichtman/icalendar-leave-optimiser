#![allow(dead_code, unused_imports, unused_variables, unreachable_code)]

use chrono::{Duration, NaiveDate, Utc};

extern crate clap;
use clap::{arg, command, Arg, ArgGroup, Parser};

use log::{debug, error, info, warn};

use std::fs;
use std::path::PathBuf;

// TODO: Review modules, imports, structure. Potentially add back in library definition to Cargo.toml
mod cal_opt;
use crate::cal_opt::cal_opt;

fn main() {
    let cli = Cli::parse();
    let log_level = match cli.verbose {
        0 => log::Level::Error,
        1 => log::Level::Warn,
        2 => log::Level::Info,
        _ => log::Level::Debug,
    };
    simple_logger::init_with_level(log_level).expect("Error initialising logging, aborting.");

    debug!("{cli:#?}");
    let cal_opt = cal_opt(cli.file, cli.duration);
    debug!("{cal_opt:#?}");
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

/*
I was hoping to use argument relations to narrow the api but most inputs we can work with.
Ze question is, do we want to?
I think for now we'll stick to duration only and assume today.
We can work on input date validation and argument grouping later

Argument behaviour matrix:

| start | end | duration | Behaviour
| 0 | 0 | 0 | Default to today + 365
| 0 | 0 | 1 | Default to today + duration
| 0 | 1 | 0 | Default to today until end
| 0 | 1 | 1 | Set start to end minus duration?
| 1 | 0 | 0 | Invalid? Default to start + 365?
| 1 | 0 | 1 | Set to start + duration
| 1 | 1 | 0 | Set to start + end
| 1 | 1 | 1 | If duration mismatch, error, else continue

*/
