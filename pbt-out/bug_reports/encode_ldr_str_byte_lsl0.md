# Bug: encode_ldr_str encodes byte LSL #0 with S=0 instead of S=1
**Law:** For a byte-sized register-offset LDR/STR, an explicit `lsl #0` must set the S bit (bit 12), matching llvm-mc/gas (`strb w0, [x0, x0, lsl #0]` → 0x38207800).
**Impact:** Object files diverge from GNU as / llvm-mc on the S bit of byte-sized register-offset stores/loads when the assembler text writes `lsl #0`. Disassembly shows `[Xn, Xm]` instead of `[Xn, Xm, lsl #0]`.
**Function:** encode_ldr_str
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:33
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** `encode_ldr_str([Reg("w0"), MemRegOffset{base:"x0", index:"x0", extend:Some("lsl"), shift:Some(0)}], is_load=false, size=0, is_signed=false, is_128bit=false)`
**Expected:** `Ok(Word(0x38207800))` (S=1)
**Actual:** `Ok(Word(0x38206800))` (S=0)
**Severity:** low
**Root cause:** load_store.rs:179-181 sets S=1 only when `shift_amount > 0`. For size=00 the architectural shift is 0, so llvm-mc uses S=1 to record an explicit `#0` while the SUT treats Some(0) like an omitted shift.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:180`
```rust
                    let s_val = if shift_amount > 0 { 1u32 } else { 0u32 };
                    (0b011u32, s_val)
```
**Suggested fix:** Set S=1 when a shift amount is present (including `#0` on byte accesses), matching gas/llvm-mc.
```rust
                    let s_val = if shift.is_some() { 1u32 } else { 0u32 };
                    (0b011u32, s_val)
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_regression_byte_lsl0 -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `941647872`,
 right: `941651968`: regoff mismatch for strb w0, [x0, x0, lsl #0]
minimal failing input: is_load = false, size = 0, rt = 0, rn = 0, rm = 0, w_index = false, ext_sel = 0, s_bit = 1
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
