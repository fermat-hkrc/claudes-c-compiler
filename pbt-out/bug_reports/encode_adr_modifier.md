# Bug: encode_adr accepts :lo12:/:got: modifiers as a bare ADR reloc
**Law:** GNU `adr` does not take `:lo12:` / `:got:` / `:got_lo12:` modifiers. llvm-mc rejects `adr x0, :lo12:foo` with "unexpected adr label". encode_adr must Err on Modifier operands. AdrPrelLo21 is the reloc for a bare symbol, not a :lo12: addend.
**Impact:** `adr x0, :lo12:foo` is encoded as WordWithReloc { word: 0x10000000, reloc_type: AdrPrelLo21, symbol: "foo", addend: 0 }. The linker applies R_AARCH64_ADR_PREL_LO21 (S+A-P) instead of rejecting the illegal modifier, producing a wrong address.
**Function:** encode_adr
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("x0"), Modifier { kind: "lo12", symbol: "foo" }]
**Additional witness:** operands = [Reg("x0"), ModifierOffset { kind: "lo12", symbol: "foo", offset: 0 }] (property encode_adr_neg_modifier_offset)
**Expected:** Err (ADR does not take :lo12: modifiers)
**Actual:** Ok(WordWithReloc { word: 0x10000000, reloc_type: AdrPrelLo21, symbol: "foo", addend: 0 })
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_adr_pbt::test_encode_adr_regression_modifier ; test_encode_adr_regression_modifier_offset
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_adr_neg_modifier -- --test-threads=1; cargo test --lib encode_adr_neg_modifier_offset -- --test-threads=1)
