# Bug: encode_ldr_str accepts SP as Rt and classifies it as SIMD
**Law:** SP is not a valid Rt for LDR/STR/LDRB/STRB/LDRH/STRH. llvm-mc/gas reject `strb sp, [x0]`; Rt=31 is WZR/XZR, never SP.
**Impact:** `strb sp, [x0]` (and `ldr sp, ...`) assembles instead of erroring. Because `is_fp_reg("sp")` is true (`s` prefix), the V bit is set and the word is a SIMD store (0x3D00001F), not even a GPR STR with Rt=31.
**Function:** encode_ldr_str
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:33
**Detected by:** Negative/error contract — llvm-mc rejects SP as Rt
**Minimal input:** `encode_ldr_str([Reg("sp"), Mem{base:"x0", offset:0}], is_load=false, size=0, is_signed=false, is_128bit=false)`
**Expected:** `Err(...)`
**Actual:** `Ok(Word(0x3D00001F))`
**Severity:** medium
**Root cause:** load_store.rs:39-43 calls `is_fp_reg` on the Rt name. `is_fp_reg` treats any name starting with `s` as SIMD, so `"sp"` sets V=1. `parse_reg_num("sp")` yields 31. There is no check that Rt is not SP.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:39`
```rust
    let fp = is_fp_reg(operands.first().map(|o| match o { Operand::Reg(r) => r.as_str(), _ => "" }).unwrap_or(""));
```
**Suggested fix:** Reject SP/WSP as Rt; do not treat `"sp"` as an S-register.
```rust
    let rt_name = match operands.first() {
        Some(Operand::Reg(r)) => r.as_str(),
        _ => "",
    };
    if rt_name.eq_ignore_ascii_case("sp") || rt_name.eq_ignore_ascii_case("wsp") {
        return Err("ldr/str: SP is not a valid Rt".to_string());
    }
    let fp = is_fp_reg(rt_name);
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_regression_sp_dest -- --test-threads=1
```
**Raw output:**
```text
Test failed: SP dest must Err (llvm-mc: invalid operand); got Ok(Word(1023410207))
minimal failing input: is_load = false, size = 0, rt = 0, rn = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
