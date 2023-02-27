use crate::{indent, types::Node};

// TODO: Finish implementing this function.
// Ideally this should be done after typechecking is done;
//      although the C compiler already typechecks for us
pub fn compile(ast: Node) -> String {
    use Node::*;

    match &ast {
        Program { body } => body
            .iter()
            .map(|stmt| compile(stmt.to_owned()))
            .collect::<Vec<_>>()
            .join("\n"),
        Function { name, params, body } => {
            let mut output = String::new();

            let params = params
                .iter()
                .map(|p| compile(p.clone()))
                .collect::<Vec<_>>()
                .join(", ");

            output.extend(format!("void {}({}) {{\n", name, params).chars());

            output.extend(
                indent(
                    &body
                        .iter()
                        .map(|stmt| compile(stmt.to_owned()))
                        .collect::<Vec<_>>()
                        .join(";\n"),
                    4,
                )
                .chars(),
            );

            output.push('}');

            output
        }
        NumericLiteral { val, typ } => format!("({}){}", typ, val),
        BinaryExpr {
            left,
            operator,
            right,
        } => format!(
            "{} {} {};",
            compile(*left.clone()),
            operator,
            compile(*right.clone())
        ),
        Identifier { name } => name.to_owned(),
        TypedIdentifier { name, typ } => format!("{}: {}", name, compile(*typ.clone())),
        CallExpr { callee, args } => format!(
            "{}({})",
            compile(*callee.clone()),
            args.iter()
                .map(|arg| compile(arg.clone()))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        _ => unimplemented!("{:?} is not implemented yet", ast),
    }
}
