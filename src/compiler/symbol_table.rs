use std::collections::HashMap;

use crate::{errors::HydorError, type_checker::type_checker::Type, utils::Span};

#[derive(Clone)]
pub enum SymbolScope {
    Global,
}

#[derive(Clone)]
pub struct Symbol {
    pub name: String,
    pub index: usize,
    pub scope: SymbolScope,
    pub span: Span,
    pub symbol_type: Type,
}

#[derive(Clone)]
pub struct SymbolTable {
    pub store: HashMap<String, Symbol>,
    pub num_definitions: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            num_definitions: 0,
        }
    }

    pub fn declare_identifier(
        &mut self,
        name: String,
        span: Span,
        symbol_type: Type,
    ) -> Result<usize, HydorError> {
        // Unreachable unless theres a type checker bug
        if let Ok(val) = self.resolve_identifier(&name, span) {
            return Err(HydorError::VariableRedeclaration {
                name: name.clone(),
                original_span: val.span,
                redeclaration_span: span,
            });
        }

        let index = self.num_definitions;
        let symbol = Symbol {
            name: name.clone(),
            scope: SymbolScope::Global, // Will be dynamic once scopes are introduced ---
            span,
            symbol_type,
            index,
        };

        self.store.insert(name, symbol);
        self.num_definitions += 1;

        Ok(index)
    }

    pub fn resolve_identifier(&mut self, name: &String, span: Span) -> Result<&Symbol, HydorError> {
        // Unreachable unless theres a type checker bug
        if !self.store.contains_key(name) {
            return Err(HydorError::UndefinedIdentifier {
                ident_name: name.to_string(),
                span: span,
            });
        }

        let value = self.store.get(name).unwrap();
        Ok(value)
    }
}
