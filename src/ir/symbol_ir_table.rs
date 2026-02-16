use std::collections::HashMap;

use crate::{
    error_handler::{error_collector::ErrorCollector, errors::VynError},
    type_checker::type_checker::Type,
    utils::Span,
};

pub enum SymbolScope {
    Register(u8),
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
    next_register: u8,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbol_scopes: vec![HashMap::new()],
            scope_depth: 0,
            next_register: 0,
        }
    }

    pub fn declare_ident_with_register(
        &mut self,
        symbol_type: Type,
        name: String,
        mutable: bool,
        register: u8,
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

        self.current_scope().insert(
            name.clone(),
            Symbol {
                symbol_type,
                scope: SymbolScope::Register(register),
                name,
                span,
                mutable,
            },
        );

        Some(())
    }

    pub fn declare_ident(
        &mut self,
        symbol_type: Type,
        name: String,
        mutable: bool,
        span: Span,
        error_collector: &mut ErrorCollector,
    ) -> Option<u8> {
        if self.current_scope().contains_key(&name) {
            let original_span = self.current_scope().get(&name).unwrap().span;
            error_collector.add(VynError::VariableRedeclaration {
                name,
                original_span,
                redeclaration_span: span,
            });
            return None;
        }

        let reg = self.allocate_register();

        self.current_scope().insert(
            name.clone(),
            Symbol {
                symbol_type,
                scope: SymbolScope::Register(reg),
                name,
                span,
                mutable,
            },
        );

        Some(reg)
    }

    pub fn enter_scope(&mut self) {
        self.symbol_scopes.push(HashMap::new());
        self.scope_depth += 1;
    }

    pub fn exit_scope(&mut self) {
        if self.scope_depth > 0 {
            // Free registers used in this scope
            if let Some(scope) = self.symbol_scopes.last() {
                for symbol in scope.values() {
                    if let SymbolScope::Register(_) = symbol.scope {
                        // Registers will be reused in parent scope
                        if self.next_register > 0 {
                            self.next_register -= 1;
                        }
                    }
                }
            }

            self.symbol_scopes.pop();
            self.scope_depth -= 1;
        }
    }

    fn allocate_register(&mut self) -> u8 {
        let reg = self.next_register;
        self.next_register += 1;
        reg
    }

    fn current_scope(&mut self) -> &mut HashMap<String, Symbol> {
        &mut self.symbol_scopes[self.scope_depth]
    }

    pub fn resolve_symbol(
        &self,
        name: &str,
        resolver_span: Span,
        error_collector: &mut ErrorCollector,
    ) -> Option<&Symbol> {
        // Search from current scope backwards to global
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
