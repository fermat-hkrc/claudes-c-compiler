# Bug: encode_adrp rejects :got:symbol+addend
**Law:** `adrp Xd, :got:sym+addend` is valid GNU/llvm-mc syntax and must emit AdrGotPage21 with that addend
**Impact:** Parser-produced ModifierOffset { kind: "got", ... } (from `:got:foo+8`) is rejected, so GOT-relative ADRP with a non-zero addend cannot be assembled even though gas and llvm-mc emit R_AARCH64_ADR_GOT_PAGE with addend 8
**Function:** encode_adrp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:653
**Detected by:** Algebraic — Invariant (4d)
**Minimal input:** encode_adrp([Reg("x0"), ModifierOffset { kind: "got", symbol: "g0", offset: 0 }])
**Expected:** Ok(WordWithReloc { word: 0x90000000, reloc_type: AdrGotPage21, symbol: "g0", addend: 0 })
**Actual:** Err("adrp needs symbol operand, got Some(ModifierOffset { kind: \"got\", symbol: \"g0\", offset: 0 })")
**Severity:** medium
**Root cause:** load_store.rs:657 matches only Operand::Modifier { kind == "got" } and falls through to the default Err for ModifierOffset, even when kind is "got"
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:657`
```rust
        Some(Operand::Modifier { kind, symbol }) if kind == "got" => {
            // adrp x0, :got:symbol
            let word = (1u32 << 31) | (0b10000 << 24) | rd;
            return Ok(EncodeResult::WordWithReloc {
                word,
                reloc: Relocation {
                    reloc_type: RelocType::AdrGotPage21,
                    symbol: symbol.clone(),
                    addend: 0,
                },
            });
        }
```
**Suggested fix:** Accept ModifierOffset with kind "got" and pass the offset through as the reloc addend
```rust
        Some(Operand::Modifier { kind, symbol }) if kind == "got" => {
            (symbol.clone(), 0i64, RelocType::AdrGotPage21)
        }
        Some(Operand::ModifierOffset { kind, symbol, offset }) if kind == "got" => {
            (symbol.clone(), *offset, RelocType::AdrGotPage21)
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_adrp_regression_got_modifier_offset -- --test-threads=1
```
**Raw output:**
```text
Test failed: ModifierOffset-got expected WordWithReloc AdrGotPage21, got Err("adrp needs symbol operand, got Some(ModifierOffset { kind: \"got\", symbol: \"g0\", offset: 0 })").
minimal failing input: rd = 0, suffix = 0, addend = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_adrp_pbt.rs
