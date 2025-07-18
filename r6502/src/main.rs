pub mod scenario_util;

mod cli;
mod test_scenarios;

fn main() -> anyhow::Result<()> {
    crate::cli::run()
}
