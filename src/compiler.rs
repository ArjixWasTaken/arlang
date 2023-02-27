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

            output.extend(format!("void {}() {{\n", name).chars());

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
        _ => unimplemented!("{:?} is not implemented yet", ast),
    }
}
