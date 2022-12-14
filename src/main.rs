use core::panic;

#[derive(Debug)]
struct Token {
    name: String,
    value: String,
}

fn lex(text: &str) {
    let chars = text.chars().collect::<Vec<char>>();
    let mut tokens: Vec<Token> = vec![];
    let mut idx = 0;

    while idx < chars.len() {
        let char: char = chars[idx];
        let val: String = char.into();
        
        let tok = match char {
            '(' => Some(Token { name: "OpenParen".into(), value: val }),
            ')' => Some(Token { name: "CloseParen".into(), value: val }),
            '[' => Some(Token { name: "OpenBracket".into(), value: val }),
            ']' => Some(Token { name: "CloseBracket".into(), value: val }),
            '{' => Some(Token { name: "OpenBrace".into(), value: val }),
            '}' => Some(Token { name: "CloseBrace".into(), value: val }),
            ',' => Some(Token { name: "Comma".into(), value: val }),
            '-' => Some(Token { name: "Operator".into(), value: val }),
            '+' => Some(Token { name: "Operator".into(), value: val }),
            '*' => Some(Token { name: "Operator".into(), value: val }),
            '%' => Some(Token { name: "Operator".into(), value: val }),
            '"' => {
                let mut string_chars: Vec<char> = vec![];
                idx += 1;
                let mut prev = chars.get(idx).unwrap_or(&'.');

                while idx < chars.len() {
                    if chars[idx] == '"' {
                        if prev != &'\\' {
                            break;
                        }
                    }

                    if prev == &'\\' {
                        match chars[idx] {
                            'n' => {
                                string_chars.pop();
                                string_chars.push('\n');
                            },
                            't' => {
                                string_chars.pop();
                                string_chars.push('\t');
                            },
                            'r' => {
                                string_chars.pop();
                                string_chars.push('\r');
                            },
                            '"' => {
                                string_chars.pop();
                                string_chars.push('"');
                            },
                            _ => string_chars.push(chars[idx])
                        }
                    } else { string_chars.push(chars[idx]) }

                    prev = &chars[idx];
                    idx += 1;
                }

                if chars[idx] != '"' {
                    panic!("Missing quote from string: Did you forget to add a closing quote to the string?")
                }
                
                Some(Token{name: "String".into(), value: string_chars.iter().collect()})
            },
            '/' => {
                if let Some(next) = chars.get(idx+1) {
                    if next == &'*' || next == &'/' {
                        let mut comment_chars: Vec<char> = vec![];
                        let is_multiline = next == &'*';

                        idx += 2;
                        if idx >= chars.len() { unreachable!() }
                        let mut prev = chars.get(idx).unwrap_or(&'.');

                        while idx < chars.len() &&
                            (
                                (is_multiline && (prev != &'*' && chars[idx] != '/')) ||
                                (!is_multiline && chars[idx] != '\n')
                            ) {
                            comment_chars.push(chars[idx]);
                            prev = &chars[idx];
                            idx  += 1;
                        }

                        if is_multiline {
                            if chars[idx-1] == '*' && chars[idx] == '/' {
                                comment_chars.pop();
                                idx += 1;
                            } else {
                                panic!("Multiline comment was not closed.")
                            }
                        }
                        idx -= 1;
                        
                        Some(Token{name: "Comment".into(), value: comment_chars.into_iter().collect()})
                    } else {
                        Some(Token { name: "Operator".into(), value: val })
                    }
                } else {
                    todo!()
                }
            },
            _ => {
                if chars[idx].is_numeric() {
                    let mut number_chars: Vec<char> = vec![chars[idx]];
                    let mut is_float = false;
                    idx += 1;

                    while idx < chars.len() && (chars[idx].is_numeric() || chars[idx] == '.') {
                        if chars[idx] == '.' && is_float {
                            idx -= 1;
                            is_float = false;

                            number_chars.pop();
                            break;
                        } else if chars[idx] == '.' {
                            is_float = true;
                        }

                        number_chars.push(chars[idx]);
                        idx += 1;
                    }

                    idx -= 1;

                    Some(Token{name: (if is_float {"Float"} else {"Int"}).to_string(), value: number_chars.into_iter().collect()})
                } else if idx+1 < chars.len() && chars[idx] == '.' && chars[idx+1] == '.' {
                    idx += 1;
                    Some(Token{name: "Operator".into(), value: "RangeOperator".into()})
                } else if chars[idx].is_whitespace() {
                    None
                } else {
                    panic!("Unexpected value found: {}", chars[idx..].into_iter().take(20).collect::<String>())
                }
            },
        };
        
        idx += 1;
        
        if let Some(tok) = tok {
            tokens.push(tok)
        }

        println!("Stack left: {}", chars[idx..].iter().collect::<String>());
    }

    println!("{:#?}", tokens);
}

fn repl() {
    loop {
        let input: String = casual::input().default("".into()).prompt("> ").get();

        if input.trim().len() == 0 || input.trim() == "exit" {
            println!("Exiting...");
            break;
        }

        lex(&input);
    }
}

fn main() -> std::io::Result<()>{
    let args: Vec<String> = std::env::args().collect();

    println!("arlang -- version 0.0.1 // Github: ArjixWasTaken");
    if args.contains(&"repl".into()) {
        return Ok(repl())
    }
    
    let input = "1 + 1";
    print!("> {input}\n");
    lex(input);

    Ok(())
}
