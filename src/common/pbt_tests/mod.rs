//! Property-based test modules (proptest) for src/common.
//! Linked in from src/common/mod.rs via `#[cfg(test)] mod pbt_tests;`.
mod encoding_pbt;
mod const_arith_pbt;
mod types_pbt;
