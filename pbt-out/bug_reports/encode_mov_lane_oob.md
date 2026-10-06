# Bug: encode_mov wraps out-of-range NEON lane indices
**Law:** `mov v0.b[16], w0` is invalid; llvm-mc requires a lane in [0, 15] for .b
**Impact:** An out-of-range lane is masked (`index & 0xF`) and encoded as lane 0, so a bounds error becomes a silent insert into the wrong element
**Function:** encode_mov
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:36
**Detected by:** Negative/error contract — llvm-mc lane-range rejection
**Minimal input:** encode_mov([RegLane { reg: "v0", elem_size: "b", index: 16 }, Reg("w0")])
**Expected:** Err
**Actual:** Ok(Word(0x4e010c00)) — INS V0.B[0], W0
**Severity:** high
**Root cause:** data_processing.rs:36 uses `*index & 0xF` (and similar masks for h/s/d) instead of rejecting an index above the element-size maximum
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:36`
```rust
            "b" => ((*index & 0xF) << 1) | 0b00001,
            "h" => ((*index & 0x7) << 2) | 0b00010,
            "s" => ((*index & 0x3) << 3) | 0b00100,
            "d" => ((*index & 0x1) << 4) | 0b01000,
```
**Suggested fix:** Error when `index` exceeds the per-size maximum instead of masking.
```rust
            "b" if *index <= 15 => ((*index) << 1) | 0b00001,
            "h" if *index <= 7 => ((*index) << 2) | 0b00010,
            "s" if *index <= 3 => ((*index) << 3) | 0b00100,
            "d" if *index <= 1 => ((*index) << 4) | 0b01000,
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_regression_lane_oob -- --test-threads=1
```
**Raw output:**
```text
lane OOB must Err, got Ok(Word(1308695552))
minimal failing input: vd = 0, rm = 0, idx = 16
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mov_pbt.rs
