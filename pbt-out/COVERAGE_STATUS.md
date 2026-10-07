# Coverage Status

**Mode:** incremental (standard tier)
**Target:** encode_bgez (pseudo.rs:309)
**Evidence level:** file-level (symbol presence) — `coverage_gaps` reported no line-level .gcda/.profraw and listed encode_bgez as NOT LINKED in C++ test binaries (irrelevant here). Rust `cargo test --lib encode_bgez` linked and executed the production symbol (14 tests).

## This campaign
- Function encode_bgez exercised by 14 tests in encode_bgez_pbt.rs
- 11 passing properties + 1 passing KAT; 1 failing property + 1 failing regression witness
- Documented behaviors covered: llvm-mc differential, BGE rs,x0 expansion metamorphic, B-type layout, ABI/xN/fp alias, Symbol/Label/Reg/Imm targets, Imm-as-rs, arity/invalid rs/target negatives, extra-operand negative (bug)

## Contract-surface sweep
- coverage_gaps called once (standard tier 1/1)
- No additional documented branch without a property; remaining gap is the confirmed extra-operand bug
