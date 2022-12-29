use std::borrow::Borrow;
use crate::types::{Program, Token, TokeType, Node};

pub struct Parser<'a> {
    pub tokens: &'a mut Vec<Token>
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

impl<'ctx> Parser<'ctx> {
    pub fn new(tokens: &'ctx mut Vec<Token>) -> Self {
        Self { tokens }
    }

    fn reached_eof(&'ctx self) -> bool {
        self.tokens.len()  > 0
    }

    pub fn parse(&'ctx mut self) -> Node {
        let mut program = Node::Program{body: vec![]};

        if let Node::Program { body } = &mut program {
            while !self.reached_eof() {
                if let Some(expr) = self.parse_additive_expr() {
                    body.push(expr);
                }
            }
        }

        program
    }

    fn at(&'ctx self) -> Option<&'ctx Token> {
        Some(self.tokens.first()?)
    }

    fn consume(&'ctx mut self) -> Option<Token> {
        Some(self.tokens.remove(0))
    }

    fn expect(&'ctx mut self, typ: TokeType) -> Option<Token> {
        let tok = self.consume()?;
        if !matches!(&tok.typ, typ) {
            panic!("lol, this might be a parser bug")
        }
        Some(tok)
    }

    fn parse_primary_expr(&'ctx mut self) -> Option<Node> {
        use TokeType::*;

        match self.at()?.typ {
            Identifier => Some(Node::Identifier { name: self.consume()?.val.into() }),
            Int => Some(Node::NumericLiteral { typ: "int".into(), val: self.consume()?.val.into() }),
            Float => Some(Node::NumericLiteral { typ: "float".into(), val: self.consume()?.val.into() }),
            OpenParen => {
                let val = self.parse_additive_expr();
                self.expect(CloseParen);
                val
            }
            _ => panic!("wtf")
        }
    }

    fn parse_additive_expr(&'ctx mut self) -> Option<Node> {
        let mut Left = empty_bin_expr();

        let leftExpr = self.parse_multiplicative_expr();

        match &Left {
            Node::BinaryExpr { left, right, operator } => {
                let leftExpr = self.parse_primary_expr();

                while matches!(self.at()?.typ, TokeType::Operator) {
                    let op = &self.expect(TokeType::Operator)?.val;
                    let right = self.parse_multiplicative_expr();

                    if operator == "" {
                        Left = Node::BinaryExpr { left: leftExpr?.into(), right: right?.into(), operator: operator.into() };
                    } else {
                        Left = Node::BinaryExpr { left: Left.into(), right: right?.into(), operator: operator.into() };
                    }
                }

                return Some(Left);
            }
            _ => unreachable!()
        }
    }

    fn parse_multiplicative_expr(&'ctx mut self) -> Option<Node> {
        let mut Left = empty_bin_expr();

        match &Left {
            Node::BinaryExpr { left, right, operator } => {
                let leftExpr = self.parse_primary_expr();

                while matches!(self.at()?.typ, TokeType::Operator) {
                    let op = &self.expect(TokeType::Operator)?.val;
                    let right = self.parse_primary_expr();

                    if operator == "" {
                        Left = Node::BinaryExpr { left: leftExpr?.into(), right: right?.into(), operator: operator.into() };
                    } else {
                        Left = Node::BinaryExpr { left: Left.into(), right: right?.into(), operator: operator.into() };
                    }
                }

                return Some(Left);
            }
            _ => unreachable!()
        }
    }
}
