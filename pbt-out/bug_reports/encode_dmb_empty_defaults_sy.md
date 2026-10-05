# Bug: encode_dmb encodes omitted option as SY
**Law:** encode_dmb([]) must return Err, matching GNU as ("missing immediate expression at operand 1") and llvm-mc ("too few operands for instruction")
**Impact:** A `dmb` with no operand, which both GNU as and llvm-mc reject, is encoded as `dmb sy`. Invalid assembly is accepted and becomes a full-system barrier.
**Function:** encode_dmb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:6
**Detected by:** Negative/Error Contract (omitted operand; llvm-mc and GNU as reject)
**Minimal input:** encode_dmb(&[])
**Expected:** Err
**Actual:** Ok(Word(0xd5033fbf))  // dmb sy
**Severity:** medium
**Root cause:** system.rs:23 `_ => 0b1111` — the empty-slice arm of `operands.first()` is `None`, which is not Barrier/Symbol, so the function defaults CRm to SY instead of returning Err.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:23`
```rust
        _ => 0b1111,
```
**Suggested fix:** Return Err when the operand list is empty.
```rust
        None => return Err("dmb requires a barrier option".to_string()),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dmb_regression_empty -- --test-threads=1 --nocapture
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_dmb_pbt::encode_dmb_neg_empty' panicked at src/backend/arm/assembler/encoder/encode_dmb_pbt.rs:264:1:
Test failed: empty operands must Err (gas/llvm-mc reject omitted dmb option) at src/backend/arm/assembler/encoder/encode_dmb_pbt.rs:381.
minimal failing input: _n = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_dmb_pbt.rs
