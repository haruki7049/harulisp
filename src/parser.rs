use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/harulisp.pest"]
pub struct HarulispParser;

#[derive(Debug, Default)]
pub struct SExpression {
    function_word: String,
    arguments: Vec<String>,
}

fn parse_pair(pair: Pair<Rule>) -> anyhow::Result<SExpression> {
    match pair.as_rule() {
        Rule::EOI | Rule::word | Rule::PUNCT_CHARACTER => unreachable!(),
        Rule::SExpression => {
            dbg!(pair.clone().tokens());

            let mut rule = pair.into_inner();
            let function_word = rule.next().unwrap();

            let arguments: Vec<String> = vec![];

            Ok(SExpression {
                function_word: String::from(function_word.as_span().as_str()),
                arguments,
            })
        }
    }
}

pub fn parse(s: &str) -> anyhow::Result<SExpression> {
    let mut pairs = HarulispParser::parse(Rule::SExpression, s)?;

    parse_pair(pairs.next().unwrap())
}
