mod args;
mod single_step_tests;

fn main() -> anyhow::Result<()> {
    use args::{Args, Command};
    use clap::Parser;

    match Args::parse().command {
        Command::Run { filter } => crate::single_step_tests::run_scenarios_with_filter(&filter)?,
        Command::RunJson { json } => crate::single_step_tests::run_scenario_from_json(&json)?,
    }
    Ok(())
}
