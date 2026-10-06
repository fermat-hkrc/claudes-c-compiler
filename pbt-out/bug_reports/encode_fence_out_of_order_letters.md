# Bug: encode_fence accepts out-of-order and duplicate iorw letters
**Law:** Fence letter operands must be letters selected in-order from iorw (llvm-mc: `operand must be formed of letters selected in-order from 'iorw' or be 0`); duplicates and permutations such as `wroi` / `ii` / `irow` must be rejected
**Impact:** Mistyped fence arguments still assemble. `fence wroi, iorw` encodes as a full barrier; `fence ii, rw` encodes as `fence i, rw`. The parser's is_fence_arg accepts any i/o/r/w combo of length 1..=4, so the bad strings are caller-reachable
**Function:** encode_fence
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:5
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fence([FenceArg("wroi"), FenceArg("i")])
**Expected:** Err
**Actual:** Ok(Word) with pred bits 0xF (contains i,o,r,w regardless of order)
**Severity:** medium
**Root cause:** encoder/mod.rs:446-449 parse_fence_bits uses `str::contains` per letter, so order, duplicates, and extra permutations still set the same bits; encode_fence never validates the string
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/mod.rs:446`
```rust
    if s.contains('i') { bits |= 8; }
    if s.contains('o') { bits |= 4; }
    if s.contains('r') { bits |= 2; }
    if s.contains('w') { bits |= 1; }
```
**Suggested fix:** Accept only in-order subsequences of "iorw" (and Imm 0); otherwise Err
```rust
fn parse_fence_bits(s: &str) -> Result<u32, String> {
    const ORDER: &[u8] = b"iorw";
    let b = s.as_bytes();
    if b.is_empty() || b.len() > 4 {
        return Err(format!("invalid fence operand: {s}"));
    }
    let mut i = 0usize;
    let mut bits = 0u32;
    for &c in b {
        while i < 4 && ORDER[i] != c { i += 1; }
        if i >= 4 { return Err(format!("invalid fence operand: {s}")); }
        bits |= 8 >> i;
        i += 1;
    }
    Ok(bits)
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fence_neg_out_of_order -- --test-threads=1
```
**Raw output:**
```text
Test failed: out-of-order/duplicate/uppercase wroi must Err (llvm-mc in-order iorw); got Ok(Word(260046863))
minimal failing input: pred = "wroi", succ = "i"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fence_pbt.rs
