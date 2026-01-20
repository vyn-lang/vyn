use std::collections::HashMap;

use crate::{
    errors::{ErrorCollector, VynError},
    type_checker::type_checker::Type,
    utils::Span,
};

pub struct SymbolType {
    pub symbol_type: Type,
    pub span: Span,
}

pub struct SymbolTypeTable {
    store: HashMap<String, SymbolType>,
}

impl SymbolTypeTable {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn declare_identifier(
        &mut self,
        ident: String,
        t: Type,
        span: Span,
        errors: &mut ErrorCollector,
    ) -> Result<(), ()> {
        if let Some(existing) = self.store.get(&ident) {
            errors.add(VynError::VariableRedeclaration {
                name: ident,
                original_span: existing.span,
                redeclaration_span: span,
            });
            return Err(());
        }

        let symbol_type = SymbolType {
            symbol_type: t,
            span,
        };

        self.store.insert(ident, symbol_type);
        Ok(())
    }

    pub fn resolve_identifier(
        &self,
        ident: &str,
        span: Span,
        errors: &mut ErrorCollector,
    ) -> Result<Type, ()> {
        match self.store.get(ident) {
            Some(s) => Ok(s.symbol_type.clone()),
            None => {
                errors.add(VynError::UndefinedVariable {
                    name: ident.to_string(),
                    span,
                });
                Err(())
            }
        }
    }
}
