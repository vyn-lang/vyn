use std::collections::HashMap;

use crate::{
    ast::{
        ast::{Expr, Expression},
        type_annotation::TypeAnnotation,
    },
    errors::VynError,
    parser::parser::Parser,
    tokens::TokenType,
};

pub struct TypeTable {
    pub aliases: HashMap<String, TypeAnnotation>,
}

impl TypeTable {
    pub fn new() -> Self {
        Self {
            aliases: HashMap::new(),
        }
    }
}

impl Parser {
    pub(crate) fn try_parse_type(&mut self) -> Option<TypeAnnotation> {
        let current_token = self.current_token();
        let current_token_type = current_token.token.get_token_type();

        // check if is identifier
        if current_token_type != TokenType::Identifier {
            self.errors.add(VynError::ExpectedType {
                got: current_token_type,
                span: current_token.span,
            });
            self.advance(); // consume bad tokn
            return None;
        }

        // take type name
        let type_name = match &current_token.token {
            crate::tokens::Token::Identifier(name) => name,
            _ => unreachable!("Already checked it's an Identifier"),
        };

        let type_annotation = match TypeAnnotation::from_identifier(type_name) {
            Some(t) => {
                self.advance();
                Some(t.clone())
            }
            None => {
                let al_type = self.type_table.aliases.get(type_name)?.clone();
                self.advance();
                Some(al_type)
            }
        };

        type_annotation
    }

    pub fn enroll_type_alias(
        &mut self,
        ident: Expression,
        aliased_type: TypeAnnotation,
    ) -> Option<()> {
        let name = match ident.node {
            Expr::Identifier(s) => s,
            _ => unreachable!(),
        };

        self.type_table.aliases.insert(name, aliased_type)?;
        Some(())
    }
}
