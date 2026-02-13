use std::collections::HashMap;

use crate::{
    error_handler::{error_collector::ErrorCollector, errors::VynError},
    type_checker::type_checker::Type,
    utils::Span,
};

pub enum SymbolScope {
    Global(usize), // global idx
}

pub struct Symbol {
    pub name: String,
    pub symbol_type: Type,
    pub mutable: bool,
    pub scope: SymbolScope,
}

#[derive(Default)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
    collection_count: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            collection_count: 0,
        }
    }

    pub fn declare_ident(&mut self, symbol_type: Type, name: String, mutable: bool) {
        self.symbols.insert(
            name.clone(),
            Symbol {
                symbol_type,
                scope: SymbolScope::Global(self.collection_count),
                name,
                mutable,
            },
        );

        self.collection_count += 1;
    }

    pub fn resolve_symbol(
        &mut self,
        name: &str,
        resolver_span: Span,
        error_collector: &mut ErrorCollector,
    ) -> Option<&Symbol> {
        if let Some(symbol) = self.symbols.get(name) {
            return Some(symbol);
        }

        error_collector.add(VynError::UndefinedVariable {
            name: name.to_string(),
            span: resolver_span,
        });
        return None;
    }
}
