use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Stag;

impl Stag {
    pub fn parse_source(source: String) {
        let parse = Self::parse(Rule::program, &source);
        println!("{:?}", parse);
    }
}