use crate::types::{Node, Program, TokeType, Token};
use anyhow::{bail, Result};

pub struct Parser {
    pub tokens: Vec<Token>,
}

fn empty_bin_expr() -> Node {
    Node::BinaryExpr {
        left: Box::from(Node::BinaryExpr {
            left: Box::new(Node::MemberExpr),
            right: Box::new(Node::MemberExpr),
            operator: "".to_string(),
        }),
        right: Box::new(Node::MemberExpr),
        operator: "".to_string(),
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&mut self) -> Node {
        let mut program = Node::Program { body: vec![] };
        let Node::Program { mut body } = program else {
            unreachable!()
        };

        while !self.eof() {
            body.push(self.parse_additive_expr().unwrap());
        }

        Node::Program {
            body: body.to_vec(),
        }
    }

    fn eof(&self) -> bool {
        self.tokens.len() == 0
    }

    fn at(&mut self) -> Result<&Token> {
        let first = self.tokens.first();
        println!("At token: {:?}", first);

        first.ok_or_else(|| anyhow::anyhow!("Unexpected end of input"))
    }

    fn consume(&mut self) -> Result<Token> {
        let tok = self.at()?.clone();
        // println!("Consuming token: {:?}", tok);
        self.tokens = self.tokens[1..].to_vec();
        Ok(tok)
    }

    fn expect(&mut self, typ: TokeType) -> Result<Token> {
        let tok = self.consume()?;
        if !matches!(&tok.typ, typ) {
            bail!("lol, this might be a parser bug");
        }
        Ok(tok)
    }

    fn parse_primary_expr(&mut self) -> Result<Node> {
        use TokeType::*;

        let node = self.consume()?;

        match &node.typ {
            Identifier => Ok(if matches!(self.at()?.typ, Assignment) {
                let name = node.val;
                let op = self.expect(Assignment)?;

                let val = self.parse_additive_expr()?;
                Node::BinaryExpr {
                    left: Box::new(Node::Variable { name }),
                    right: Box::new(val),
                    operator: op.val.into(),
                }
            } else {
                Node::Identifier {
                    name: node.val.into(),
                }
            }),
            Int => Ok(Node::NumericLiteral {
                typ: "int".into(),
                val: node.val.into(),
            }),
            Float => Ok(Node::NumericLiteral {
                typ: "float".into(),
                val: node.val.into(),
            }),
            OpenParen => {
                let val = self.parse_additive_expr();
                self.expect(CloseParen)?;
                val
            }
            Keyword => {
                let name = node.val;
                match name.as_str() {
                    "const" | "let" => {
                        let name = self.expect(Identifier)?.val;
                        let op = self.expect(Assignment)?;
                        assert!(op.val == "=", "Only '=' is allowed when declaring a variable, what were you thinking?");

                        let val = self.parse_additive_expr()?;
                        Ok(Node::BinaryExpr {
                            left: Box::new(Node::Variable { name }),
                            right: Box::new(val),
                            operator: op.val.into(),
                        })
                    }
                    _ => bail!("Unexpected keyword: {}", name),
                }
            }
            _ => {
                bail!("Unexpected token: {:?}", node);
            }
        }
    }

    fn parse_additive_expr(&mut self) -> Result<Node> {
        let mut left = self.parse_multiplicative_expr()?;

        while !self.eof()
            && matches!(self.at()?.typ, TokeType::Operator)
            && ["+", "-"].contains(&self.at()?.val.as_str())
        {
            let op = &self.expect(TokeType::Operator)?.val;
            let right = self.parse_multiplicative_expr()?;

            left = Node::BinaryExpr {
                left: left.clone().into(),
                right: right.into(),
                operator: op.into(),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative_expr(&mut self) -> Result<Node> {
        let mut left = self.parse_primary_expr()?;

        while !self.eof()
            && matches!(self.at()?.typ, TokeType::Operator)
            && ["*", "/", "%"].contains(&self.at()?.val.as_str())
        {
            let op = &self.expect(TokeType::Operator)?.val;
            let right = self.parse_primary_expr()?;

            left = Node::BinaryExpr {
                left: left.clone().into(),
                right: right.into(),
                operator: op.into(),
            };
        }

        Ok(left)
    }
}
