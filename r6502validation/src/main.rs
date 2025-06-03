mod args;

fn main() -> anyhow::Result<()> {
    use args::{Args, Command};
    use clap::Parser;
    use r6502lib::single_step_tests::Scenario;

    match Args::parse().command {
        Command::Run { filter } => Scenario::run_scenarios_with_filter(&filter)?,
        Command::RunJson { json } => {
            let scenario = Scenario::from_json(&json)?;
            println!("{scenario}");
            let (result, final_state) = scenario.run();
            if let Some(final_state) = final_state {
                println!("Actual:\n{final_state}");
            } else {
                println!("Actual: (not available)");
            }
            if result {
                println!("Scenario passed")
            } else {
                println!("Scenario failed")
            }
        }
    }
    Ok(())
}
