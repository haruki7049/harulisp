# harulisp

My lisp implementation.

# Purpose of this repo

Purpose of this repo is to create my original LISP implementation. this project is only used pure rust-lang, stable toolchain, standard library and 2021 Edition.

# goal of this project

- SIMPLE IS THE BEST, but useful for your development
- nice package manager
- REPL and compiler. First, we create REPL, and then compiler
- NOT Embedded environment
- standard library, included Internet, stdout/in and etc
- test module in default

# Documents

## Default commands

- define (Bind a number or lambda to constant)
- if (fork program's stream by inputed boolean)
- lambda (a function no named, requires 0 or more arguments and an S-expression to execute)

The below commands requires 2 arguments which must be integer or float, and return number

- + (addition)
- - (substruction)
- * (multiplication)
- / (division)

The below commands requires 2 arguments which must not be List and lambda, and return boolean

- < (if left argument is greater than right argument, this S-expression return true)
- > (if right argument is greater than left argument, this S-expression return true)
- = (if left argument is equal to right argument, this S-expression return true)
- != (if left argument is not equal to right argument, this S-expression return true)

## Data type

- List
- lambda
- integer
- float
- boolean
- string

## TODO list

- [ ] write anything in this TODO list
