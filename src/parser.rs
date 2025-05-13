use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/harulisp.pest"]
pub struct HarulispParser;

#[derive(Debug, Default)]
pub struct Program {
    statements: Vec<Token>,
}

#[derive(Debug)]
pub enum Token {
    String(String),
    Word(String),
    Int(i32),
    SExpression(Vec<Token>),
}

fn parse_pair(pair: Pair<Rule>) -> anyhow::Result<Program> {
    match pair.as_rule() {
        Rule::EOI
        | Rule::Comment
        | Rule::Comments
        | Rule::punct_word
        | Rule::SExpression
        | Rule::SExpressions
        | Rule::left_parenthesis
        | Rule::right_parenthesis
        | Rule::word
        | Rule::int
        | Rule::string => unreachable!(),
        Rule::program => {
            let rule = pair.into_inner();
            let mut result: Program = Program::default();

            // TODO: Make a loop to process the program as: `( foofoo )\n( barbar )`
            rule.clone().for_each(|sexpr| {
                match sexpr.as_rule() {
                    Rule::SExpression => result.statements.push(parse_sexp(sexpr).unwrap()),
                    Rule::EOI => return,
                    Rule::Comment => (),
                    Rule::program
                    | Rule::Comments
                    | Rule::SExpressions
                    | Rule::punct_word
                    | Rule::left_parenthesis
                    | Rule::right_parenthesis
                    | Rule::word
                    | Rule::int
                    | Rule::string => unreachable!(),
                }
            });

            Ok(result)
        }
    }
}

fn parse_sexp(sexpr: Pair<Rule>) -> anyhow::Result<Token> {
    let mut result: Vec<Token> = Vec::new();

    let mut rule = sexpr.into_inner();
    let _left_parenthesis: Pair<Rule> = rule.next().unwrap(); // "("
    let mut words: Vec<Pair<Rule>> = rule.into_iter().collect(); // "defvar"
    let _right_parenthesis: Pair<Rule> = words.pop().unwrap(); // ")"

    for w in words {
        match w.as_rule() {
            Rule::SExpression => result.push(parse_sexp(w)?),
            Rule::word => result.push(Token::Word(String::from(w.as_span().as_str()))),
            Rule::string => result.push(Token::String(String::from(w.as_span().as_str()))),
            Rule::int => result.push(Token::Int(w.as_span().as_str().parse::<i32>()?)),
            Rule::Comment => (),

            Rule::EOI
            | Rule::Comments
            | Rule::SExpressions
            | Rule::program
            | Rule::punct_word
            | Rule::left_parenthesis
            | Rule::right_parenthesis => unreachable!(),
        };
    }

    Ok(Token::SExpression(result))
}

pub fn parse(s: &str) -> anyhow::Result<Program> {
    let mut pairs = HarulispParser::parse(Rule::program, s)?;

    parse_pair(pairs.next().unwrap())
}
