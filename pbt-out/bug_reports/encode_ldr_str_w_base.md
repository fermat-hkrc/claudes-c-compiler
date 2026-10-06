# Bug: encode_ldr_str accepts a W-register memory base
**Law:** The base of LDR/STR is Xn or SP, never Wn/WSP. llvm-mc/gas reject `ldr x0, [w0]`.
**Impact:** `ldr x0, [w0]` silently encodes as `ldr x0, [x0]` (0xF9400000). A 32-bit base is accepted as if it were 64-bit.
**Function:** encode_ldr_str
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:33
**Detected by:** Negative/error contract — llvm-mc rejects W base
**Minimal input:** `encode_ldr_str([Reg("x0"), Mem{base:"w0", offset:0}], is_load=true, size=0b11, is_signed=false, is_128bit=false)`
**Expected:** `Err(...)`
**Actual:** `Ok(Word(0xF9400000))` — same as `ldr x0, [x0]`
**Severity:** medium
**Root cause:** `parse_reg_num("w0")` returns 0 with no width check; encode_ldr_str uses that number as Rn.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:50`
```rust
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
```
**Suggested fix:** Require a 64-bit base name (Xn, SP, or LR).
```rust
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let b = base.to_ascii_lowercase();
            if !(b.starts_with('x') || b == "sp" || b == "lr") {
                return Err("ldr/str: base must be Xn or SP".to_string());
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_regression_w_base -- --test-threads=1
```
**Raw output:**
```text
LDR X0, [W0] must Err; base must be Xn|SP (llvm-mc rejects it)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
