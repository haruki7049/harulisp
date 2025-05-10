use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/harulisp.pest"]
pub struct HarulispParser;

#[derive(Debug)]
pub struct SExpression;

fn parse_pair(_pair: Pair<Rule>) -> anyhow::Result<SExpression> {
    todo!();
}

pub fn parse(s: &str) -> anyhow::Result<SExpression> {
    let mut pairs = HarulispParser::parse(Rule::SExpression, s)?;

    parse_pair(pairs.next().unwrap())
}
