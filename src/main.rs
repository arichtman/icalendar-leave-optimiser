#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unreachable_code,
    unused_mut
)]

use demo::create_and_write_demo_calendar;
use procedure::do_procedure;

extern crate clap;
use clap::{arg, command, Parser};

use icalendar::Calendar;
use log::debug;

use std::path::PathBuf;

mod data_transforms;
mod demo;
mod logic;
mod procedure;

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    debug!("{cli:#?}");
    let log_level = match cli.verbose {
        0 => log::Level::Error,
        1 => log::Level::Warn,
        2 => log::Level::Info,
        _ => log::Level::Debug,
    };
    simple_logger::init_with_level(log_level).expect("Error initialising logging, aborting.");

    let _procedure_out = do_procedure(cli.file, cli.duration)?;

    logic::do_logic(&[false; 1]);
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
    duration: u64,
}

#[cfg(test)]
mod tests;
