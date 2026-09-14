# Bug: encode_ldrsw rejects LDRSW (literal)
**Law:** LDRSW Xt, label is a valid ARM addressing mode (opc=10 011 000 imm19 Rt) and llvm-mc / gas assemble it with an LD_PREL_LO19 fixup. The encoder must emit WordWithReloc { RelocType::Ldr19 }.
**Impact:** `ldrsw x0, foo` fails to assemble (`unsupported ldrsw operands`). Sibling encode_ldr_str already handles Operand::Symbol for `ldr`. Inline assembly and handwritten `.s` files that PC-relative-load a signed word cannot be encoded.
**Function:** encode_ldrsw
**Detected by:** Algebraic — Invariant (literal field layout) / ARM ARM LDRSW (literal)
**Minimal input:** `[Reg("x0"), Symbol("foo")]`
**Expected:** Ok(WordWithReloc { word: 0x98000000, reloc: Ldr19, symbol: "foo", addend: 0 })
**Actual:** Err("unsupported ldrsw operands: [Reg(\"x0\"), Symbol(\"foo\")]")
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrsw_regression_literal
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrsw -- --test-threads=1`
