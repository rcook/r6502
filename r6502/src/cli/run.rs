use crate::cli::Args;
use crate::cli::Command::{Debug, Run, Validate, ValidateJson};
use anyhow::Result;
use clap::Parser;
use r6502lib::emulator::{run_scenario, run_scenarios_with_filter};
use r6502lib::terminal::run_terminal;
use r6502lib::tui::run_gui;
use r6502lib::validation::Scenario;

pub fn run() -> Result<()> {
    match Args::parse().command {
        Debug { path, load, start } => run_gui(&path, load, start)?,
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
