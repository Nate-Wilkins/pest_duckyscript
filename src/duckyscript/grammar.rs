extern crate pest;

#[derive(Parser)]
#[grammar = "duckyscript/grammar.pest"]
pub struct ParserDuckyScript;
