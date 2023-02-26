#[derive(Debug, Copy, Clone)]
pub enum TokeType {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Comma,
    Operator,
    Assignment,
    String,
    Int,
    Float,
    Comment,
    Identifier,
    Keyword,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub typ: TokeType,
    pub val: String,
}

#[derive(Debug, Clone)]
pub enum Node {
    Program {
        body: Vec<Node>,
    },
    MemberExpr,
    CallExpr,
    NumericLiteral {
        typ: String,
        val: String,
    },
    Identifier {
        name: String,
    },
    VariableDecl {
        modifier: String,
        name: String,
    },
    BinaryExpr {
        left: Box<Node>,
        right: Box<Node>,
        operator: String,
    },
}

impl Node {
    pub fn get_operator(&self) -> Option<String> {
        match self {
            Node::BinaryExpr { operator, .. } => Some(operator.clone()),
            _ => None,
        }
    }
}

pub struct Statement {
    pub(crate) kind: Node,
}

pub struct Program {
    pub body: Vec<Statement>,
}

pub struct Expr {
    pub stmt: Statement,
}

pub trait NewBinaryExpr {
    fn new(left: Expr, right: Expr, op: String) -> Self;
}

// impl NewBinaryExpr for BinaryExpr {
//     fn new(left: Expr, right: Expr, operator: String) -> Self {
//         BinaryExpr {
//             expr: Expr { stmt: Statement { kind: Node::BinaryExpr } },
//             left,
//             right,
//             operator
//         }
//     }
// }

pub struct MemberExpr {
    pub expr: Expr,
    pub object: Expr,
    pub property: Expr,
    pub computed: bool,
}

pub struct CallExpr {
    pub expr: Expr,
    pub caller: Expr,
    pub arguments: Vec<Expr>,
}

pub struct Identifier {
    pub expr: Expr,
    pub symbol: String,
}

pub struct VariableDecl {
    pub exp: Expr,
    pub symbol: String,
}

pub struct IntLiteral {
    pub expr: Expr,
    pub value: i64,
}

pub struct FloatLiteral {
    pub expr: Expr,
    pub value: f64,
}
