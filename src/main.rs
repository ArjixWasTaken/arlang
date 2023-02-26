#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

extern crate core;

mod lexer;
mod parser;
mod types;

use crate::types::Node;
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

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    println!(
        "{} -- version {} {} Github: {}",
        "arlang".green(),
        "0.0.1".yellow(),
        "//".bright_black(),
        "ArjixWasTaken".bright_blue()
    );
    if false {
        // || args.contains(&"repl".into()) {
        return Ok(repl());
    }

    let input = "1 + 1 + 1 / 5";
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
