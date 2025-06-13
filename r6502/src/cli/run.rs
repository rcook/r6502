use crate::cli::Args;
use crate::cli::Command::{Debug, Run, Validate, ValidateJson};
use crate::terminal::run_terminal;
use crate::tui::run_gui;
use anyhow::Result;
use clap::Parser;
use r6502lib::validation::Scenario;
use r6502lib::{run_scenario, run_scenarios_with_filter};

pub(crate) fn run() -> Result<()> {
    match Args::parse().command {
        Debug { path, load, start } => run_gui(&path, load, start)?,
        Run(opts) => run_terminal(&opts)?,
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
