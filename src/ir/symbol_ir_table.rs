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
    // Track the highest register used at each scope level
    scope_register_watermarks: Vec<u8>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbol_scopes: vec![HashMap::new()],
            scope_depth: 0,
            next_register: 0,
            scope_register_watermarks: vec![0],
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

        // Update watermark if necessary
        if register >= self.next_register {
            self.next_register = register + 1;
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
        // Save the current register watermark
        self.scope_register_watermarks.push(self.next_register);
    }

    pub fn exit_scope(&mut self) {
        if self.scope_depth > 0 {
            self.symbol_scopes.pop();
            self.scope_depth -= 1;

            // Restore the register watermark from before this scope
            if let Some(watermark) = self.scope_register_watermarks.pop() {
                self.next_register = watermark;
            }
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
