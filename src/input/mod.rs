#[derive(PartialEq)]
enum TerminalStatement {
    REPL,
    //Stdout,
}

#[allow(dead_code)]
fn check_in_repl(terminal_statement: TerminalStatement) -> bool {
    if terminal_statement == TerminalStatement::REPL {
        true
    } else {
        false
    }
}
