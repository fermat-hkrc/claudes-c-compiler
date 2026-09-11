# Bug: encode_bl accepts :lo12: / modifier operands as Call26 symbols
**Law:** BL operand is a label or encodable integer PC offset; encode_bl([Modifier{kind,symbol}]) and encode_bl([ModifierOffset{...}]) must be Err.
**Impact:** `bl :lo12:foo` (and `:lo12:foo+N`) is encoded as an ordinary Call26 reloc to `foo`, dropping the modifier. llvm-mc does not treat that as BL. The resulting object has the wrong relocation class relative to the source text.
**Function:** encode_bl
**Detected by:** Negative/Error Contract
**Minimal input:** Modifier { kind: "lo12", symbol: "foo" }
**Expected:** Err
**Actual:** Ok(WordWithReloc { word: 0x94000000, reloc: Call26, symbol: "foo", addend: 0 }) — get_symbol returns (symbol, 0) and discards kind.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs `test_encode_bl_regression_modifier`
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_bl_pbt -- --test-threads=1 reproduced the failure.
