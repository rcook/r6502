#![cfg(test)]

use crate::single_step_tests::Scenario;
use anyhow::Result;
use rstest::rstest;

// Scenario "b4 e9 4b": actual value $F5 (245) for register y does not match expected value $06 (6) (r6502lib\src\single_step_tests\scenario.rs:201)
#[rstest]
#[case(r#"{ "name": "61 1e 49", "initial": { "pc": 26086, "s": 108, "a": 250, "x": 117, "y": 104, "p": 173, "ram": [ [26086, 97], [26087, 30], [26088, 73], [30, 225], [147, 188], [148, 211], [54204, 79]]}, "final": { "pc": 26088, "s": 108, "a": 160, "x": 117, "y": 104, "p": 45, "ram": [ [30, 225], [147, 188], [148, 211], [26086, 97], [26087, 30], [26088, 73], [54204, 79]]}, "cycles": [ [26086, 97, "read"], [26087, 30, "read"], [30, 225, "read"], [147, 188, "read"], [148, 211, "read"], [54204, 79, "read"]] }"#)]
#[case(r#"{ "name": "61 8b 47", "initial": { "pc": 8970, "s": 138, "a": 190, "x": 116, "y": 121, "p": 169, "ram": [ [8970, 97], [8971, 139], [8972, 71], [139, 215], [255, 241], [0, 87], [22513, 19]]}, "final": { "pc": 8972, "s": 138, "a": 56, "x": 116, "y": 121, "p": 169, "ram": [ [0, 87], [139, 215], [255, 241], [8970, 97], [8971, 139], [8972, 71], [22513, 19]]}, "cycles": [ [8970, 97, "read"], [8971, 139, "read"], [139, 215, "read"], [255, 241, "read"], [0, 87, "read"], [22513, 19, "read"]] }"#)]
#[case(r#"{ "name": "b4 e9 4b", "initial": { "pc": 39850, "s": 107, "a": 12, "x": 102, "y": 180, "p": 111, "ram": [ [39850, 180], [39851, 233], [39852, 75], [233, 245], [79, 6]]}, "final": { "pc": 39852, "s": 107, "a": 12, "x": 102, "y": 6, "p": 109, "ram": [ [79, 6], [233, 245], [39850, 180], [39851, 233], [39852, 75]]}, "cycles": [ [39850, 180, "read"], [39851, 233, "read"], [233, 245, "read"], [79, 6, "read"]] }"#)]
fn basics(#[case] json: &str) -> Result<()> {
    let scenario = Scenario::from_json(json)?;
    let (result, final_state) = scenario.run();
    if !result {
        println!("{scenario}");
        if let Some(final_state) = final_state {
            println!("Actual:\n{final_state}");
        } else {
            println!("Actual: (not available)");
        }
    }
    assert!(result);
    Ok(())
}
