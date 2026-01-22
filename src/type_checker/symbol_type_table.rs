use std::collections::HashMap;

use crate::{
    errors::{ErrorCollector, VynError},
    type_checker::type_checker::Type,
    utils::Span,
};

pub struct SymbolType {
    pub symbol_type: Type,
    pub span: Span,
    pub mutable: bool,
}

pub struct SymbolTypeTable {
    store: HashMap<String, SymbolType>,
    type_aliases: HashMap<String, SymbolType>,
}

impl SymbolTypeTable {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            type_aliases: HashMap::new(),
        }
    }

    pub fn declare_identifier(
        &mut self,
        ident: String,
        t: Type,
        span: Span,
        mutable: bool,
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
            mutable,
        };

        self.store.insert(ident, symbol_type);
        Ok(())
    }

    pub fn resolve_identifier(
        &self,
        ident: &str,
        span: Span,
        errors: &mut ErrorCollector,
    ) -> Result<&SymbolType, ()> {
        match self.store.get(ident) {
            Some(s) => Ok(s),
            None => {
                // Check type alias instead
                let al_type = self.type_aliases.get(ident);

                if al_type.is_none() {
                    errors.add(VynError::UndefinedVariable {
                        name: ident.to_string(),
                        span,
                    });
                    return Err(());
                }

                Ok(al_type.unwrap())
            }
        }
    }

    pub fn enroll_type_alias(
        &mut self,
        name: String,
        an_type: Type,
        span: Span,
    ) -> Result<(), VynError> {
        if self.type_aliases.contains_key(&name) {
            return Err(VynError::TypeAliasRedeclaration { name, span });
        }

        let symbol = SymbolType {
            symbol_type: an_type,
            mutable: false,
            span,
        };

        self.type_aliases.insert(name, symbol);
        Ok(())
    }
}
