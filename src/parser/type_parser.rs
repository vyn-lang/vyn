use std::collections::HashMap;

use crate::{
    ast::{
        ast::{Expr, Expression},
        type_annotation::TypeAnnotation,
    },
    error_handler::errors::VynError,
    parser::{lookups::Precedence, parser::Parser},
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

        // Dispatch table for special type syntax
        match current_token_type {
            TokenType::LeftBracket => self.parse_array_type(),
            _ => self.parse_simple_type(),
        }
    }

    fn parse_simple_type(&mut self) -> Option<TypeAnnotation> {
        let current_token = self.current_token();
        let current_token_type = current_token.token.get_token_type();

        // check if is identifier
        if current_token_type != TokenType::Identifier {
            self.errors.add(VynError::ExpectedType {
                got: current_token_type,
                span: current_token.span,
            });
            self.advance(); // consume bad token
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
                // Check if it's a type alias
                if let Some(al_type) = self.type_table.aliases.get(type_name) {
                    let result = al_type.clone();
                    self.advance();
                    Some(result)
                } else {
                    self.errors.add(VynError::InvalidTypeName {
                        got: type_name.clone(),
                        span: current_token.span,
                    });
                    self.advance();
                    None
                }
            }
        };

        type_annotation
    }

    fn parse_array_type(&mut self) -> Option<TypeAnnotation> {
        if !self.expect(TokenType::LeftBracket) {
            return None;
        }

        if self.current_token_type() == TokenType::RightBracket {
            self.advance();

            let arr_type = self.try_parse_type()?;
            let arr = TypeAnnotation::SequenceType(Box::new(arr_type));

            return Some(arr);
        }

        let size = self.try_parse_expression(Precedence::Default.into())?;

        if !self.expect(TokenType::RightBracket) {
            return None;
        }

        let arr_type = self.try_parse_type()?;

        let arr = TypeAnnotation::ArrayType(Box::new(arr_type), size);

        Some(arr)
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
