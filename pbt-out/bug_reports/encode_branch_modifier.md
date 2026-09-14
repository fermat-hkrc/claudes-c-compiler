# Bug: encode_branch accepts :lo12: / modifier operands as Jump26 symbols
**Law:** B operand is a label or encodable integer PC offset; encode_branch([Modifier{kind,symbol}]) and encode_branch([ModifierOffset{...}]) must be Err.
**Impact:** `b :lo12:foo` (and `:lo12:foo+N`) is encoded as an ordinary Jump26 reloc to `foo`, dropping the modifier. llvm-mc does not treat that as B. The resulting object has the wrong relocation class relative to the source text.
**Function:** encode_branch
**Detected by:** Negative/Error Contract
**Minimal input:** Modifier { kind: "lo12", symbol: "foo" }
**Expected:** Err
**Actual:** Ok(WordWithReloc { word: 0x14000000, reloc: Jump26, symbol: "foo", addend: 0 }) — get_symbol returns (symbol, 0) and discards kind.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs `test_encode_branch_regression_modifier`
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_branch_pbt -- --test-threads=1 reproduced the failure.
