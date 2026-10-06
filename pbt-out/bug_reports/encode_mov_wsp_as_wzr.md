# Bug: encode_mov treats WSP as WZR (ORR) instead of ADD #0
**Law:** `mov Wsp, Wn` / `mov Wn, Wsp` must encode as ADD (immediate) #0, the same encoding llvm-mc and GNU as emit for MOV to/from SP
**Impact:** 32-bit stack-pointer moves assemble as writes to WZR, so `mov w0, wsp` clobbers the wrong register and `mov wsp, w0` does not update WSP
**Function:** encode_mov
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:129
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** encode_mov([Reg("w0"), Reg("wsp")])
**Expected:** Word(0x110003e0) — ADD W0, WSP, #0 (`mov w0, wsp`)
**Actual:** Word(0x2a1f03e0) — ORR W0, WZR, WZR (`mov w0, wzr`)
**Severity:** high
**Root cause:** data_processing.rs:129 compares only the exact name `sp`, so `wsp` falls through to the ORR-XZR path where register 31 means WZR
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:129`
```rust
        if rd_name.to_lowercase() == "sp" || rm_name.to_lowercase() == "sp" {
            let sf = sf_bit(is_64);
            // ADD Xd, Xn, #0: sf 0 0 10001 00 imm12=0 Rn Rd
            let word = ((sf << 31) | (0b10001 << 24)) | (rm << 5) | rd;
            return Ok(EncodeResult::Word(word));
        }
```
**Suggested fix:** Treat `wsp` the same as `sp` (and keep width from the register prefix).
```rust
        let rd_l = rd_name.to_lowercase();
        let rm_l = rm_name.to_lowercase();
        if rd_l == "sp" || rd_l == "wsp" || rm_l == "sp" || rm_l == "wsp" {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_regression_wsp -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: mov w0, wsp must match ADD WSP encoding 0x110003e0
  left: 706675680
 right: 285213664
minimal failing input: rd = 31, rm = 0, is_64 = false, rd_sp = true, rm_sp = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mov_pbt.rs
