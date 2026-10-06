# Bug: encode_ldr_str treats XZR/x31 as SP for the base register
**Law:** Rn=31 in LDR/STR (immediate/register) is SP, not XZR. llvm-mc/gas reject `ldr x0, [xzr]` and `ldr x0, [x31]`.
**Impact:** `ldr x0, [xzr]` silently encodes as `ldr x0, [sp]` (0xF94003E0). Callers that pass XZR as a base get a stack-pointer load/store instead of an assembler error.
**Function:** encode_ldr_str
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:33
**Detected by:** Negative/error contract — llvm-mc rejects XZR as base
**Minimal input:** `encode_ldr_str([Reg("x0"), Mem{base:"xzr", offset:0}], is_load=true, size=0b11, is_signed=false, is_128bit=false)`
**Expected:** `Err(...)`
**Actual:** `Ok(Word(0xF94003E0))` — same as `ldr x0, [sp]`
**Severity:** medium
**Root cause:** `parse_reg_num` maps both `"sp"` and `"xzr"`/`"x31"` to 31, and encode_ldr_str does not distinguish them for the base.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:50`
```rust
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
```
**Suggested fix:** Reject XZR/WZR/x31/w31 as a memory base (Rn=31 is SP only when the name is SP/WSP).
```rust
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            if base.eq_ignore_ascii_case("xzr")
                || base.eq_ignore_ascii_case("wzr")
                || base.eq_ignore_ascii_case("x31")
                || base.eq_ignore_ascii_case("w31")
            {
                return Err("ldr/str: base must be Xn or SP, not XZR".to_string());
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_regression_xzr_base -- --test-threads=1
```
**Raw output:**
```text
LDR X0, [XZR] must Err; Rn=31 is SP not XZR (llvm-mc rejects it)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
