mod args;

fn main() -> anyhow::Result<()> {
    use args::{Args, Command};
    use clap::Parser;
    use r6502lib::single_step_tests::Scenario;

    match Args::parse().command {
        Command::Run {
            report_path,
            filter,
        } => Scenario::run_scenarios_with_filter(&report_path, &filter)?,
        Command::RunJson { json } => {
            let scenario = Scenario::from_json(&json)?;
            println!("{scenario}");
            let (result, final_state) = scenario.run();
            if result {
                println!("Scenario passed")
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
