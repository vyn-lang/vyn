use std::collections::HashMap;

use crate::{
    error_handler::{
        error_collector::{self, ErrorCollector},
        errors::VynError,
    },
    type_checker::type_checker::Type,
    utils::Span,
};

pub enum SymbolScope {
    Global(usize), // global idx
}

pub struct Symbol {
    pub name: String,
    pub symbol_type: Type,
    pub span: Span,
    pub mutable: bool,
    pub scope: SymbolScope,
}

#[derive(Default)]
pub struct SymbolTable {
    pub symbol_scopes: Vec<HashMap<String, Symbol>>,
    scope_depth: usize,
    collection_count: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbol_scopes: vec![HashMap::new()], // Global scope
            collection_count: 0,
            scope_depth: 0,
        }
    }

    pub fn declare_ident(
        &mut self,
        symbol_type: Type,
        name: String,
        mutable: bool,
        span: Span,
        error_collector: &mut ErrorCollector,
    ) -> Option<()> {
        if self.current_scope().contains_key(&name) {
            let original_span = self.current_scope().get(&name).unwrap().span;
            error_collector.add(VynError::VariableRedeclaration {
                name,
                original_span,
                redeclaration_span: span,
            });

            return None;
        }

        self.current_scope_push(
            name.clone(),
            Symbol {
                symbol_type,
                scope: SymbolScope::Global(self.collection_count),
                name,
                span,
                mutable,
            },
        );

        self.collection_count += 1;
        Some(())
    }

    pub fn enter_scope(&mut self) {
        self.symbol_scopes.push(HashMap::new());
        self.scope_depth += 1;
    }

    pub fn exit_scope(&mut self) {
        if self.scope_depth > 0 {
            self.symbol_scopes.pop();
            self.scope_depth -= 1;
        }
    }

    fn current_scope(&mut self) -> &mut HashMap<String, Symbol> {
        &mut self.symbol_scopes[self.scope_depth]
    }

    fn current_scope_push(&mut self, k: String, v: Symbol) {
        self.current_scope().insert(k, v);
    }

    pub fn resolve_symbol(
        &self,
        name: &str,
        resolver_span: Span,
        error_collector: &mut ErrorCollector,
    ) -> Option<&Symbol> {
        for scope in self.symbol_scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }

        // Not found in any scope
        error_collector.add(VynError::UndefinedVariable {
            name: name.to_string(),
            span: resolver_span,
        });
        None
    }
}
