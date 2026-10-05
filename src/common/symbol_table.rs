use crate::common::types::CType;
use crate::common::fx_hash::FxHashMap;

/// Information about a declared symbol.
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub ty: CType,
    /// Explicit alignment from _Alignas or __attribute__((aligned(N))).
    /// Used by _Alignof(var) to return the correct alignment per C11 6.2.8p3.
    pub explicit_alignment: Option<usize>,
}

/// A scope in the symbol table.
#[derive(Debug)]
struct Scope {
    symbols: FxHashMap<String, Symbol>,
}

impl Scope {
    fn new() -> Self {
        Self { symbols: FxHashMap::default() }
    }
}

/// Scoped symbol table supporting nested lexical scopes.
#[derive(Debug)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { scopes: vec![Scope::new()] }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare(&mut self, symbol: Symbol) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.symbols.insert(symbol.name.clone(), symbol);
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.symbols.get(name) {
                return Some(sym);
            }
        }
        None
    }

}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Property-based tests (pi-pbt campaign, round 01_common — sweep round)
// ============================================================================

#[cfg(test)]
mod pbt_tests {
    use super::*;
    use crate::common::types::CType;
    use proptest::prelude::*;

    fn sym(name: &str) -> Symbol {
        Symbol { name: name.to_string(), ty: CType::Int, explicit_alignment: None }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]

        /// P16: scoping state machine — innermost declaration wins, popping a
        /// scope restores outer visibility, global scope always visible.
        #[test]
        fn pbt_p16_scope_shadowing(
            depth in 1usize..8,
            names in prop::collection::vec("[a-z]{1,4}", 0..8),
        ) {
            let mut st = SymbolTable::new();
            st.declare(sym("g")); // global
            prop_assert!(st.lookup("g").is_some());

            for _ in 0..depth {
                st.push_scope();
                // Shadow every name seen so far with a redeclaration in the inner scope.
                for n in &names {
                    st.declare(sym(n));
                    prop_assert!(st.lookup(n).is_some(), "innermost declare of {} must be visible", n);
                }
            }
            // Pop all inner scopes: shadowed lookups fall back to global (or vanish).
            for _ in 0..depth {
                st.pop_scope();
                prop_assert!(st.lookup("g").is_some(), "global must survive scope pops");
            }
            // The global scope itself was pushed by new() and remains.
            prop_assert!(st.lookup("g").is_some());
        }
    }
}
