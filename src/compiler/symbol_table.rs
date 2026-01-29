use std::{collections::HashMap, rc::Rc};

use crate::{error_handler::errors::VynError, type_checker::type_checker::Type, utils::Span};

#[derive(Clone)]
pub struct Symbol {
    pub symbol_type: Type,
    pub register: u8,
    pub span: Span,
}

#[derive(Clone, Default)]
pub struct SymbolTable {
    store: HashMap<String, Symbol>,
    parent: Option<Rc<SymbolTable>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            parent: None,
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
        // First check current scope
        if let Some(symbol) = self.store.get(ident) {
            return Ok(symbol);
        }

        // Then check parent scopes
        if let Some(parent) = &self.parent {
            return parent.resolve_identifier(ident, span);
        }

        Err(VynError::UndefinedVariable {
            name: ident.to_string(),
            span,
        })
    }

    pub fn get_register(&self, ident: &str) -> Option<u8> {
        self.store
            .get(ident)
            .map(|s| s.register)
            .or_else(|| self.parent.as_ref().and_then(|p| p.get_register(ident)))
    }

    pub fn get_type(&self, ident: &str) -> Option<&Type> {
        self.store
            .get(ident)
            .map(|s| &s.symbol_type)
            .or_else(|| self.parent.as_ref().and_then(|p| p.get_type(ident)))
    }

    pub fn contains(&self, ident: &str) -> bool {
        self.store.contains_key(ident) || self.parent.as_ref().map_or(false, |p| p.contains(ident))
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    pub fn enter_scope(&self) -> Self {
        Self {
            store: HashMap::new(),
            parent: Some(Rc::new(self.clone())),
        }
    }

    pub fn exit_scope(self) -> Self {
        match self.parent {
            Some(parent) => (*parent).clone(),
            None => self, // If we're at the root scope, return self
        }
    }
}
