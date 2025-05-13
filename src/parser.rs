use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/harulisp.pest"]
pub struct HarulispParser;

#[derive(Debug)]
pub struct Program;

#[derive(Debug)]
pub enum Token {
    Word(String),
    Int(i32),
    SExpression(Vec<Token>),
}

fn parse_pair(pair: Pair<Rule>) -> anyhow::Result<Program> {
    match pair.as_rule() {
        Rule::EOI
        | Rule::punct_word
        | Rule::SExpression
        | Rule::left_parenthesis
        | Rule::right_parenthesis
        | Rule::word
        | Rule::int => unreachable!(),
        Rule::program => {
            let mut rule = pair.into_inner();
            let sexpr: Pair<Rule> = rule.next().unwrap();

            match sexpr.as_rule() {
                Rule::SExpression => {
                    let result = parse_sexp(sexpr)?;
                    dbg!(result);
                    todo!()
                }
                v => {
                    dbg!(v);
                    todo!()
                }
            }
        }
    }
}

fn parse_sexp(sexpr: Pair<Rule>) -> anyhow::Result<Token> {
    let result: Token = Token::SExpression(Vec::new());

    let mut rule = sexpr.into_inner();
    let _left_parenthesis: Pair<Rule> = rule.next().unwrap(); // "("
    let mut words: Vec<Pair<Rule>> = rule.into_iter().collect(); // "defvar"
    let _right_parenthesis: Pair<Rule> = words.pop().unwrap(); // ")"

    dbg!(&words);

    for w in words {
        dbg!(&w);

        match w.as_rule() {
            Rule::SExpression => {
                todo!()
            }
            Rule::word | Rule::int => {
                todo!();
            }

            Rule::EOI
            | Rule::program
            | Rule::punct_word
            | Rule::left_parenthesis
            | Rule::right_parenthesis => todo!(),
        };
    }

    Ok(result)
}

pub fn parse(s: &str) -> anyhow::Result<Program> {
    let mut pairs = HarulispParser::parse(Rule::program, s)?;

    parse_pair(pairs.next().unwrap())
}
