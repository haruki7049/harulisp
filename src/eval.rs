use crate::parser;
use crate::parser::SExpression;

pub fn eval(str: String) -> anyhow::Result<()> {
    let sexpr: SExpression = match parser::parse(str.as_str()) {
        Ok(v) => v,
        Err(e) => anyhow::bail!(e),
    };

    dbg!(sexpr);

    Ok(())
}
