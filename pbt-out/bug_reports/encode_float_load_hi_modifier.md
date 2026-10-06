# Bug: encode_float_load accepts %hi/%pcrel_hi/%tprel_hi on FLW/FLD
**Law:** For every FLW/FLD whose memory operand uses a hi-type reloc modifier (%hi, %pcrel_hi, %tprel_hi), encode_float_load must return Err. llvm-mc only allows %lo/%pcrel_lo/%tprel_lo on LOAD-FP.
**Impact:** `flw f0, %hi(foo)(x0)` is encoded as an I-type load with R_RISCV_LO12_I instead of being rejected. The linker then applies a low-12 reloc to a symbol that the author requested as a high reloc, producing a wrong address.
**Function:** encode_float_load
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:5
**Detected by:** Negative/Error Contract
**Minimal input:** encode_float_load([Reg("f0"), MemSymbol { base: "x0", symbol: "%hi(foo)", modifier: "" }], 0b010)
**Expected:** Err (llvm-mc: "operand must be a symbol with %lo/%pcrel_lo/%tprel_lo modifier or an integer in the range [-2048, 2047]")
**Actual:** Ok(WordWithReloc { word: 8199, reloc_type: Lo12I, symbol: "foo", addend: 0 })
**Severity:** high
**Root cause:** float.rs:15-18 remaps PcrelHi20 to PcrelLo12I and Hi20 to Lo12I instead of rejecting hi-type modifiers; TprelHi20 is left unchanged (`other => other`).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:16`
```rust
                RelocType::PcrelHi20 => RelocType::PcrelLo12I,
                RelocType::Hi20 => RelocType::Lo12I,
                other => other,
```
**Suggested fix:** Accept only lo12 reloc kinds on LOAD-FP; reject hi-type modifiers.
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelLo12I | RelocType::Lo12I | RelocType::TprelLo12I => reloc_type,
                other => {
                    return Err(format!("float load: unsupported reloc modifier {:?}", other));
                }
            };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_float_load_neg_hi_modifier -- --test-threads=1
cargo test --lib test_encode_float_load_regression_hi_modifier -- --test-threads=1
```
**Raw output:**
```text
Test failed: hi-type modifier %hi(foo) must Err on float load (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo); got Ok(WordWithReloc { word: 8199, reloc: Relocation { reloc_type: Lo12I, symbol: "foo", addend: 0 } }) at src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs:568.
minimal failing input: (mn, f3) = (
    "flw",
    2,
), rd = "f0", rs1 = "x0", s = "foo", hi = "%hi"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs
