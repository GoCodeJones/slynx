use crate::{
    ast::{
        ASTExpression, ASTStatment, ASTStatmentKind, ElementDeffinition, ElementDeffinitionKind,
    },
    hir::macros::{ElementMacro, StatmentMacro},
};

#[derive(Debug)]
///Macro for generating in js in place
pub struct JSMacro {}

impl StatmentMacro for JSMacro {
    fn name(&self) -> &'static str {
        "@js"
    }
    fn execute(
        &self,
        args: &Vec<crate::ast::ASTStatment>,
        statment_index: usize,
    ) -> Vec<crate::ast::ASTStatment> {
        vec![ASTStatment {
            kind: ASTStatmentKind::Expression(ASTExpression {
                kind: crate::ast::ASTExpressionKind::Identifier(statment_index.to_string()),
                span: args[0].span.clone(),
            }),
            span: args[0].span.clone(),
        }]
    }
}

impl ElementMacro for JSMacro {
    fn name(&self) -> &'static str {
        "@js"
    }
    fn execute(
        &self,
        args: &crate::ast::MacroElementArgs,
        deffinition_index: usize,
    ) -> Vec<crate::ast::ElementDeffinition> {
        match args {
            crate::ast::MacroElementArgs::Empty | crate::ast::MacroElementArgs::Deffinitions(_) => {
                vec![]
            }
            crate::ast::MacroElementArgs::Statments(s) => vec![ElementDeffinition {
                kind: ElementDeffinitionKind::RawJs("console.log('Hello world')".into()),
                span: crate::ast::Span { start: 0, end: 0 },
            }],
        }
    }
}
