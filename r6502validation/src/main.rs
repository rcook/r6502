#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::pedantic)]
#![allow(clippy::bool_to_int_with_if)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::verbose_bit_mask)]
#![allow(missing_docs)]

mod args;

fn main() -> anyhow::Result<()> {
    use args::{Args, Command};
    use clap::Parser;
    use r6502lib::{run_scenario, run_scenarios_with_filter};
    use r6502validationlib::Scenario;

    match Args::parse().command {
        Command::Run {
            report_path,
            filter,
        } => run_scenarios_with_filter(&report_path, &filter)?,
        Command::RunJson { json } => {
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
