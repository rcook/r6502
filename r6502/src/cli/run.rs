use crate::cli::Args;
use crate::cli::Command::{Debug, Run, Validate, ValidateJson};
use anyhow::Result;
use clap::Parser;
use log::LevelFilter;
use r6502lib::emulator::{run_scenario, run_scenarios_with_filter};
use r6502lib::terminal_ui::run as run_terminal;
use r6502lib::text_ui::run_tui;
use r6502lib::validation::Scenario;
use simple_logging::log_to_file;

pub fn run() -> Result<()> {
    log_to_file("r6502.log", LevelFilter::Info)?;
    log_panics::init();

    match Args::parse().command {
        Debug(opts) => run_tui(&opts.into())?,
        Run(opts) => run_terminal(&opts.into())?,
        Validate {
            report_path,
            filter,
        } => run_scenarios_with_filter(&report_path, &filter)?,
        ValidateJson { json } => {
            let scenario = Scenario::from_json(&json)?;
            println!("{scenario}");
            let (result, final_state) = run_scenario(&scenario);
            if result {
                println!("Scenario passed");
            } else {
                println!("Scenario failed");
                if let Some(final_state) = final_state {
                    println!("Actual:\n{final_state}");
                } else {
                    println!("Actual: (not available)");
                }
            }
        }
    }
    Ok(())
}
