#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

extern crate core;

mod compiler;
mod lexer;
mod parser;
mod types;

use crate::types::Node;
use anyhow::Result;
use colored::*;
use rustyline::{error::ReadlineError, DefaultEditor};

// a function to indent a multiline string
pub fn indent(s: &str, n: usize) -> String {
    let mut out = String::new();
    for line in s.lines() {
        out.push_str(&format!("{}{}\n", " ".repeat(n), line));
    }
    out
}

fn repl() {
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                if line.trim().len() == 0 || line.trim() == "exit" {
                    println!("Exiting...");
                    break;
                }

                rl.add_history_entry(line.as_str()).unwrap();
                let parser = parser::Parser::new(lexer::lex(&format!(
                    "fn main() {{\n{}\n}}",
                    indent(&line, 4)
                )))
                .parse();

                match &parser {
                    Node::Program { body } => {
                        println!("{:#?}", body);
                    }
                    _ => unreachable!(),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Exiting...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Exiting...");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next(); // skip the first argument

    println!(
        "{} -- version {} {} Github: {}",
        "arlang".green(),
        "0.0.1".yellow(),
        "//".bright_black(),
        "ArjixWasTaken".bright_blue()
    );

    let mut input = String::from("1 + 1 + 1 / 5");

    match args.next() {
        Some(arg) => match arg.as_str() {
            "repl" => return Ok(repl()),
            "compile" => {
                input = args.collect::<Vec<_>>().join(" ");
            }
            _ => {
                println!("Unknown argument: {}", arg);
                return Ok(());
            }
        },
        None => {}
    }

    print!("> {input}\n");

    let parser = parser::Parser::new(lexer::lex(&input)).parse();

    match &parser {
        Node::Program { body } => {
            let code = compiler::compile(parser);
            println!("Generated code:\n\n{}", code);
        }
        _ => unreachable!(),
    }

    Ok(())
}
