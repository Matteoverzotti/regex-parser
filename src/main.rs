mod helper;
mod nfa;
mod dfa;
mod testing;

use std::fs;
use serde_json::from_str;
use helper::parser;
use testing::{RegexTestSuite, SingleTest};

fn main() {
    let file: String = fs::read_to_string("regex_tests.json")
        .expect("Unable to read file");
    let tests: Vec<RegexTestSuite> = from_str(&file)
        .expect("JSON was not well-formatted");

    let mut total: i32 = 0;
    let mut failures: i32 = 0;

    for test in tests {
        println!("\n=== Test suite {}: `{}` ===", test.name, test.regex);

        // build the DFA once per suite
        let tokens: Vec<parser::Token>  = parser::tokenize(&test.regex);
        let tokens: Vec<parser::Token> = parser::to_postfix(tokens);
        let nfa: nfa::NFA = parser::build_nfa(tokens);
        let dfa: dfa::DFA = nfa.to_dfa();

        for SingleTest { input, expected } in &test.test_strings {
            total += 1;
            let result = dfa.accepts_word(input);
            if result != *expected {
                failures += 1;
                println!(
                    "  ❌ [FAIL] Input: {:<10} | Expected: {:<5} | Got: {}",
                    format!("\"{}\"", input),
                    expected,
                    result
                );
            } else {
                println!(
                    "  ✅ [PASS] Input: {:<10} | Result matches expected: {}",
                    format!("\"{}\"", input),
                    result
                );
            }
        }
        println!();
    }

    println!("\nRan {} tests: {} passed, {} failed\n",
             total, total - failures, failures);

    if failures > 0 {
        std::process::exit(1);
    }
}