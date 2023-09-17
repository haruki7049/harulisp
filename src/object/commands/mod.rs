/// Commands, which is used in eval module.
#[derive(Debug, PartialEq)]
enum Commands {
    Define,
    If,
    Lambda,
    Calcurator(Calcurator),
    Comparison(Comparison),
}

#[derive(Debug, PartialEq)]
enum Calcurator {
    Addition,
    Substruction,
    Multiplication,
    Division,
}

#[derive(Debug, PartialEq)]
enum Comparison {
    Greater,
    Shorter,
    Equal,
    NotEqual,
}
