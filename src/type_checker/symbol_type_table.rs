use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    error_handler::{error_collector::ErrorCollector, errors::VynError},
    type_checker::type_checker::Type,
    utils::Span,
};

#[derive(Clone)]
pub struct SymbolType {
    pub symbol_type: Type,
    pub span: Span,
    pub mutable: bool,
}

#[derive(Clone)]
pub struct SymbolTypeTable {
    pub parent: Option<Rc<SymbolTypeTable>>,
    store: HashMap<String, SymbolType>,
    type_aliases: HashMap<String, Type>,
}

impl SymbolTypeTable {
    pub fn new() -> Self {
        Self {
            parent: None,
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
        // Check current scope
        if let Some(s) = self.store.get(ident) {
            return Ok(s);
        }

        // Walk up parent scopes
        let mut current = self.parent.as_ref();
        while let Some(parent) = current {
            if let Some(s) = parent.store.get(ident) {
                return Ok(s);
            }
            current = parent.parent.as_ref();
        }

        errors.add(VynError::UndefinedVariable {
            name: ident.to_string(),
            span,
        });
        Err(())
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

        self.type_aliases.insert(name, an_type);
        Ok(())
    }

    pub fn enter_scope(&self) -> SymbolTypeTable {
        SymbolTypeTable {
            parent: Some(Rc::new(self.clone())),
            store: HashMap::new(),
            type_aliases: HashMap::new(),
        }
    }

    pub fn exit_scope(self) -> SymbolTypeTable {
        match self.parent {
            Some(parent_rc) => Rc::try_unwrap(parent_rc).unwrap_or_else(|rc| (*rc).clone()),
            None => panic!("Cannot exit global scope"),
        }
    }
}
