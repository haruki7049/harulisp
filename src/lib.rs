//! # harulisp
//! This crate contains a my dialect of LISP.
//!
//! # PURPOSE of this crate
//!
//! Purpose of this repo is to create my original LISP implementation. this project is only used pure rust-lang, stable toolchain, standard library and 2021 Edition.
//!
//! # goal of this project
//!
//! - SIMPLE IS THE BEST, but useful for your development
//! - nice package manager
//! - REPL and compiler. First, we create REPL, and then compiler
//! - NOT Embedded environment
//! - standard library, included Internet, stdout/in and etc
//! - test module in default
//!
//! # Default commands
//!
//! - define (Bind a number or lambda to constant)
//! - if (fork program's stream by inputed boolean)
//! - lambda (a function no named, requires 0 or more arguments and an S-expression to execute)
//!
//! The below commands requires 2 arguments which must be integer or float, and return number
//!
//! - plus( + ) (addition)
//! - minus( - ) (substruction)
//! - asterisk( * ) (multiplication)
//! - slash( / ) (division)
//!
//! The below commands requires 2 arguments which must not be List and lambda, and return boolean
//!
//! - shorter( < ) (if right argument is greater than left argument, this S-expression return true)
//! - greater( > ) (if left argument is greater than right argument, this S-expression return true)
//! - equal( = ) (if left argument is equal to right argument, this S-expression return true)
//! - not_equal( != ) (if left argument is not equal to right argument, this S-expression return true)
//!
//! # Data type
//!
//! - List
//! - lambda
//! - integer
//! - float
//! - boolean
//! - string
//!
//! # structure of rust module
//!
//! - src/data (contains some data, which is Token, Commands and Object)
//! - src/lexer (contains lexer, which is to convert from string literal to Token)

/// data module, contains the data which is used by lexer, parser and evaluator.
pub mod data;

/// lexer module, contains my original lexer.
pub mod lexer;
