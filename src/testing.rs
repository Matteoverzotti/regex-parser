use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegexTestSuite {
    pub name: String,
    pub regex: String,
    pub test_strings: Vec<SingleTest>,
}

#[derive(Debug, Deserialize)]
pub struct SingleTest {
    pub input: String,
    pub expected: bool,
}
