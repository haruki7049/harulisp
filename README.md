# harulisp

My lisp implementation.

## TODO list

- [x] MAKE this main structure, like REPL!!
- [x] make hexer
- [x] make parser
- [x] make evaluator
- [x] make REPL
- [ ] make standard library written in harulisp
- [ ] add more operator
- [ ] do anything for src/eval/mod.rs, 33:13, which has many operator.
- [ ] add variable's type, this expression is as this, (define i: int 3).
- [ ] assemble string literal in lexer
- [ ] assemble string literal in object
- [ ] assemble variable's float type
- [ ] write auto test for new function in lexer
- [ ] create number trait, as able to plus and minus
- [ ] the problem, (3.0) print as (3) in repl. SO DIFFICULT because I cannot understand the reason.

### command line argments parser

- [ ] make command line argments parser, as using clap crate, to make compiler or buildtool

### compiler

- [ ] make harulisp compiler, from harulisp code to native code.

### GUI client

- [ ] add GUI client? Is this really needed?

## Refactoring list

- [x] construct directry structure, as src/object/mod.rs
- [ ] rename the name which i do not like

## Idea

- [ ] this is OK? "NO Implicit type conversion"
