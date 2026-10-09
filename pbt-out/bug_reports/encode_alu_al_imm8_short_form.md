# Bug: encode_alu never emits AL/r8 accumulator short form (04+op*8 ib)
**Law:** ∀ ALU op, imm8 I, destination AL: encoding must be the accumulator short form `[0x04 + alu_op*8, I as u8]` preferred by gas/llvm-mc and documented by the dead `0x04` branch beside the EAX short form.
**Impact:** Byte-sized immediate ALU to AL is 3 bytes (`80 /r ib`) instead of 2 (`04+op*8 ib`). Semantically equivalent at run time, but breaks bit-exact agreement with llvm-mc/gas and wastes a byte on a hot path (flags-setting addb/andb/cmpb $imm, %al).
**Function:** encode_alu
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:447
**Detected by:** Differential — llvm-mc i686
**Minimal input:** `addb $1, %al` (also `addb $-128, %al`)
**Expected:** `[0x04, 0x01]`
**Actual:** `[0x80, 0xc0, 0x01]`
**Severity:** medium
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:447-450` handles all size==1 imm→reg with the general `0x80 /r` form before the short-form branch; the later `if size == 1 { 0x04 }` at line 458 is dead. EAX short form (0x05+op*8) is implemented only for size!=1 large imm.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:447`
```rust
                if size == 1 {
                    self.bytes.push(0x80);
                    self.bytes.push(self.modrm(3, alu_op, dst_num));
                    self.bytes.push(val as u8);
                } else if (-128..=127).contains(&val) {
```
**Suggested fix:** For size==1 and dst_num==0, emit `0x04 + alu_op*8` then imm8; otherwise keep `0x80 /r`.
```rust
                if size == 1 {
                    if dst_num == 0 {
                        self.bytes.push(0x04 + alu_op * 8);
                    } else {
                        self.bytes.push(0x80);
                        self.bytes.push(self.modrm(3, alu_op, dst_num));
                    }
                    self.bytes.push(val as u8);
                } else if (-128..=127).contains(&val) {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib 'backend::i686::assembler::encoder::encode_alu_pbt::encode_alu_regression_addb_al_short_form' -- --test-threads=1 --nocapture
```
**Raw output:**
```text
assertion `left == right` failed: regression: AL imm8 short form (got [80, c0, 01])
  left: [128, 192, 1]
 right: [4, 1]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_alu_pbt.rs (encode_alu_regression_addb_al_short_form)
