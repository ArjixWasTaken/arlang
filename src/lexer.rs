use crate::types::{ Token, TokeType };
use crate::types::Node::Identifier;

pub(crate) fn lex(text: &str) -> Vec<Token> {
    let chars = text.chars().collect::<Vec<char>>();
    let mut tokens: Vec<Token> = vec![];
    let mut idx = 0;

    while idx < chars.len() {
        let char: char = chars[idx];
        let val: String = char.into();

        let tok = match char {
            '(' => Some(Token { typ: TokeType::OpenParen, val }),
            '=' => Some(Token { typ: TokeType::Assignment, val }),
            ')' => Some(Token { typ: TokeType::CloseParen, val }),
            '[' => Some(Token { typ: TokeType::OpenBracket, val }),
            ']' => Some(Token { typ: TokeType::CloseBracket, val }),
            '{' => Some(Token { typ: TokeType::OpenBrace, val }),
            '}' => Some(Token { typ: TokeType::CloseBrace, val }),
            ',' => Some(Token { typ: TokeType::Comma, val }),
            '-' => Some(Token { typ: TokeType::Operator, val }),
            '*' => Some(Token { typ: TokeType::Operator, val }),
            '+' => Some(Token { typ: TokeType::Operator, val }),
            '%' => Some(Token { typ: TokeType::Operator, val }),
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
                            // Replaces some escaped characters with their actual value.
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

                Some(Token{ typ: TokeType::String, val: string_chars.iter().collect()})
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

                        Some(Token{ typ: TokeType::Comment, val: comment_chars.into_iter().collect()})
                    } else {
                        Some(Token { typ: TokeType::Operator, val })
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

                    Some(Token{
                        typ: (if is_float {TokeType::Float} else {TokeType::Int}),
                        val: number_chars.into_iter().collect()
                    })
                } else if idx+1 < chars.len() && chars[idx] == '.' && chars[idx+1] == '.' {
                    idx += 1;
                    Some(Token{ typ: TokeType::Operator, val: "Range".into()})
                } else if chars[idx].is_whitespace() {
                    None
                } else if chars[idx].is_alphabetic() || chars[idx] == '_' {
                    let mut identifier_or_keyword_chars: Vec<char> = vec![];

                    while idx < chars.len() && (chars[idx].is_alphanumeric() || chars[idx] == '_') {
                        identifier_or_keyword_chars.push(chars[idx]);
                        idx += 1;
                    }

                    idx -= 1;
                    let word = identifier_or_keyword_chars.iter().collect::<String>();

                    match word.as_str() {
                        "const" | "let" => Some(Token{typ: TokeType::Keyword, val: word}),
                        _ => Some(Token{typ: TokeType::Identifier, val: word})
                    }
                } else {
                    panic!("Unexpected value found: {}", chars[idx..].into_iter().take(20).collect::<String>())
                }
            },
        };

        idx += 1;

        if let Some(tok) = tok {
            tokens.push(tok)
        }
    }

    tokens
}
