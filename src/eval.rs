use crate::parser;
use crate::parser::Program;

pub fn eval(str: String) -> anyhow::Result<()> {
    let sexpr: Program = match parser::parse(str.as_str()) {
        Ok(v) => v,
        Err(e) => anyhow::bail!(e),
    };

    dbg!(sexpr);

    Ok(())
}
