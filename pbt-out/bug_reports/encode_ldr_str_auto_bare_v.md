# Bug: encode_ldr_str_auto encodes bare Vn as D-form instead of rejecting it
**Law:** `ldr`/`str` with a bare Vn destination (no arrangement) must be rejected; llvm-mc/gas report invalid operand
**Impact:** `ldr v0, [x0]` / `str v0, [x0]` silently assemble as 64-bit FP D-form (`ldr/str d0`). A programmer or later pass that emits V-form scalar loads gets a D-sized transfer with no diagnostic.
**Function:** encode_ldr_str_auto
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:7
**Detected by:** Negative/error contract vs llvm-mc (sweep)
**Minimal input:** encode_ldr_str_auto([Reg("v0"), Mem{base:"x0", offset:0}], is_load=false)
**Expected:** Err (llvm-mc: "error: invalid operand for instruction")
**Actual:** Ok(Word(0xfd000000)) — same bits as `str d0, [x0]`
**Severity:** medium
**Root cause:** load_store.rs:26 unknown prefixes, including `v`, take `else { 0b11 }`. `is_fp_reg` treats `v` as FP so V=1, producing D-form. There is no error path for a bare V register.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:26`
```rust
    } else {
        0b11 // default 64-bit
    };
```
**Suggested fix:** Reject `v` prefixes (bare Vn is not a scalar LDR/STR Rt) instead of defaulting to 64-bit.
```rust
    } else if reg_name.starts_with('v') {
        return Err("ldr/str: bare Vn is not a valid scalar Rt".to_string());
    } else {
        0b11 // default 64-bit
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_auto_regression_bare_v -- --test-threads=1
cargo test --lib encode_ldr_str_auto_neg_v_reg -- --test-threads=1
```
**Raw output:**
```text
Test failed: bare Vn must Err (llvm-mc: invalid operand); got Ok(Word(4244635648))
minimal failing input: is_load = false, rt = 0, rn = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs
