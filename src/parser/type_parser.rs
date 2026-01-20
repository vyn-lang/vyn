use crate::{
    ast::type_annotation::TypeAnnotation, errors::VynError, parser::parser::Parser,
    tokens::TokenType,
};

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

        match TypeAnnotation::from_identifier(type_name) {
            Some(t) => {
                self.advance();
                Some(t)
            }
            None => {
                self.errors.add(VynError::InvalidTypeName {
                    got: type_name.clone(),
                    span: current_token.span,
                });
                self.advance(); // consume bad token
                None
            }
        }
    }
}
