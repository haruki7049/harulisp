use crate::parser;
use crate::parser::Program;

pub fn eval(str: String) -> anyhow::Result<()> {
    let program: Program = match parser::parse(str.as_str()) {
        Ok(v) => v,
        Err(e) => anyhow::bail!(e),
    };

    dbg!(program);

    Ok(())
}
