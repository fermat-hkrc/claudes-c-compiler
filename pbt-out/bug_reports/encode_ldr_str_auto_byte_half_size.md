# Bug: encode_ldr_str_auto encodes Bt/Ht as 64-bit D-form
**Law:** LDR/STR of a Bt or Ht SIMD scalar must use ARM size=00 (byte) or size=01 (half) with V=1, matching llvm-mc/gas
**Impact:** Any `ldr bN` / `ldr hN` / `str bN` / `str hN` assembled by the built-in assembler is emitted as a 64-bit FP load/store (D-form). Callers that round-trip SIMD byte or halfword memory ops get the wrong opcode, scale, and transfer size.
**Function:** encode_ldr_str_auto
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:7
**Detected by:** Differential vs llvm-mc -triple=aarch64 -show-encoding (and algebraic.invariant size/V/opc)
**Minimal input:** encode_ldr_str_auto([Reg("b0"), Mem{base:"x1", offset:0}], is_load=true)
**Expected:** Word(0x3d400020) — llvm-mc `ldr b0, [x1]` (size=00 V=1 opc=01)
**Actual:** Word(0xfd400020) — same bits as `ldr d0, [x1]` (size=11 V=1 opc=01)
**Severity:** high
**Root cause:** load_store.rs:26 the size-detect chain names W/X/S/D/Q only; Bt/Ht fall into `else { 0b11 }` so they inherit 64-bit D-form. `is_128bit` is only set for `q`, so B is not distinguished from Q's size=00 path either.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:15`
```rust
    let size = if reg_name.starts_with('w') {
        0b10 // 32-bit
    } else if reg_name.starts_with('x') || reg_name == "sp" || reg_name == "xzr" || reg_name == "lr" {
        0b11 // 64-bit
    } else if reg_name.starts_with('s') {
        0b10 // 32-bit float
    } else if reg_name.starts_with('d') {
        0b11 // 64-bit float
    } else if reg_name.starts_with('q') {
        0b00 // 128-bit: size=00 with opc adjustment in encode_ldr_str
    } else {
        0b11 // default 64-bit
    };
```
**Suggested fix:** Map `h` to size=01 and `b` to size=00 (with `is_128bit` still only for `q`) before the default.
```rust
    } else if reg_name.starts_with('h') {
        0b01 // 16-bit SIMD
    } else if reg_name.starts_with('b') {
        0b00 // 8-bit SIMD
    } else if reg_name.starts_with('q') {
        0b00
    } else {
        0b11
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_auto_regression_byte_reg -- --test-threads=1
cargo test --lib encode_ldr_str_auto_diff_fp_bh -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: LDR Bt must use size=00 V=1 opc=01 (not default size=11 D-form)
  left: 4248829984
 right: 1027604512

Test failed: assertion failed: `(left == right)`
  left: `4244635648`,
 right: `1023410176`: SUT vs llvm-mc mismatch for str b0, [x0]
minimal failing input: is_load = false, rt = 0, rn = 0, imm12 = 0, is_h = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs
