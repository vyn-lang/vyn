use std::collections::HashMap;

use crate::type_checker::type_checker::Type;

enum SymbolScope {
    Global(usize), // global idx
}

pub struct Symbol {
    name: String,
    symbol_type: Type,
    mutable: bool,
    scope: SymbolScope,
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

    pub fn resolve_symbol(&mut self, name: &str) -> &Symbol {
        self.symbols.get(name).unwrap()
    }
}
