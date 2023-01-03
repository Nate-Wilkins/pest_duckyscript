extern crate pest;

#[derive(Parser)]
#[grammar = "mallardscript/grammar.pest"]
pub struct ParserMalardScript;
