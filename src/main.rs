#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unreachable_code,
    unused_mut
)]

use demo::create_and_write_demo_calendar;
use procedure::{do_procedure, CalOptProcessError};

extern crate clap;
use clap::{arg, command, Parser};

use icalendar::Calendar;
use log::debug;

use std::fs;
use std::path::PathBuf;

mod data_transforms;
mod demo;
mod logic;
mod procedure;

#[derive(Debug, thiserror::Error)]
pub enum CalOptError {
    #[error("Something went wrong")]
    GenericError,
    #[error("File reading error")]
    FileError(#[from] std::io::Error),
    #[error("Failed to parse calendar")]
    ParsingError(String),
    #[error("Failure in procedure")]
    ProcedureError(#[from] CalOptProcessError),
}

fn main() -> Result<(), CalOptError> {
    let cli = Cli::parse();
    debug!("{cli:#?}");
    let log_level = match cli.verbose {
        0 => log::Level::Error,
        1 => log::Level::Warn,
        2 => log::Level::Info,
        _ => log::Level::Debug,
    };
    simple_logger::init_with_level(log_level).expect("Error initialising logging, aborting.");

    let collated_input_calendar = collate_files(cli.file)?;
    let _procedure_out = do_procedure(collated_input_calendar, cli.duration)?;

    logic::do_logic(&[false; 1]);
    create_and_write_demo_calendar()?;
    Ok(())
}
pub fn collate_files(input_file_paths: Vec<PathBuf>) -> Result<Calendar, CalOptError> {
    let file_path = &input_file_paths[0];
    let file_contents = fs::read_to_string(file_path)?;
    // TODO: Check up error handling practices,
    //  there should be a nicer way to do this
    match file_contents.parse::<Calendar>() {
        Ok(cal) => Ok(cal),
        Err(em) => Err(CalOptError::ParsingError(em)),
    }
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
