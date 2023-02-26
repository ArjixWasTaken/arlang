#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

extern crate core;

mod lexer;
mod parser;
mod types;

use crate::types::Node;
use anyhow::Result;
use colored::*;

fn repl() {
    loop {
        let input: String = casual::input().default("".into()).prompt("> ").get();

        if input.trim().len() == 0 || input.trim() == "exit" {
            println!("Exiting...");
            break;
        }

        let parser = parser::Parser::new(lexer::lex(&input)).parse();

        match &parser {
            Node::Program { body } => {
                println!("{:#?}", body);
            }
            _ => unreachable!(),
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
            "eval" => {
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
            println!("Body {:#?}", body);
        }
        _ => unreachable!(),
    }

    Ok(())
}
