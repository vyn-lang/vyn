use std::collections::HashMap;

use crate::{error_handler::errors::VynError, type_checker::type_checker::Type, utils::Span};

pub struct Symbol {
    pub symbol_type: Type,
    pub register: u8,
    pub span: Span,
}

pub struct SymbolTable {
    store: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn declare_identifier(
        &mut self,
        ident: String,
        span: Span,
        symbol_type: Type,
        register: u8,
    ) -> Result<(), VynError> {
        if let Some(existing) = self.store.get(&ident) {
            return Err(VynError::VariableRedeclaration {
                name: ident,
                original_span: existing.span,
                redeclaration_span: span,
            });
        }

        let symbol = Symbol {
            symbol_type,
            register,
            span,
        };

        self.store.insert(ident, symbol);
        Ok(())
    }

    pub fn resolve_identifier(&self, ident: &str, span: Span) -> Result<&Symbol, VynError> {
        self.store.get(ident).ok_or(VynError::UndefinedVariable {
            name: ident.to_string(),
            span,
        })
    }

    pub fn get_register(&self, ident: &str) -> Option<u8> {
        self.store.get(ident).map(|s| s.register)
    }

    pub fn get_type(&self, ident: &str) -> Option<&Type> {
        self.store.get(ident).map(|s| &s.symbol_type)
    }

    pub fn contains(&self, ident: &str) -> bool {
        self.store.contains_key(ident)
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
}
