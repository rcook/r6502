use crate::cli::Args;
use crate::cli::Command::{Debug, Run, Validate, ValidateJson};
use crate::scenario_util;
use anyhow::Result;
use clap::Parser;
use log::LevelFilter;
use r6502ui::terminal_ui::run_terminal_ui;
use r6502ui::text_ui::run_text_ui;
use r6502validation::scenario_runner::{run_scenario, run_scenarios_with_filter};
use simple_logging::log_to_file;

pub fn run() -> Result<()> {
    log_to_file("r6502.log", LevelFilter::Info)?;
    log_panics::init();

    match Args::parse().command {
        Debug(opts) => run_text_ui(&opts.into())?,
        Run(opts) => run_terminal_ui(&opts.into())?,
        Validate {
            report_path,
            filter,
        } => run_scenarios_with_filter(&report_path, &filter)?,
        ValidateJson { json } => {
            let scenario = scenario_util::from_json(&json)?;
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
