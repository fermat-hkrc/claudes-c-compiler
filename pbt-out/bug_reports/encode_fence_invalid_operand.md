# Bug: encode_fence maps registers and non-zero immediates to iorw
**Law:** A fence operand that is not an in-order iorw letter string and not 0 must be rejected; llvm-mc reports `operand must be formed of letters selected in-order from 'iorw' or be 0` for `fence x0, x0` and `fence 1, 2`
**Impact:** `fence x0, x0` and `fence 1, rw` assemble as a full (or half-full) barrier instead of failing, so invalid assembly is emitted as a real FENCE
**Function:** encode_fence
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:5
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fence([Reg("x0"), FenceArg("rw")])
**Expected:** Err
**Actual:** Ok(Word) with pred=0xF (non-FenceArg default)
**Severity:** medium
**Root cause:** system.rs:11 — the match wildcard `_ => 0xF` treats Reg, Imm(n≠0), Symbol, Csr, Mem, and every other kind as a full iorw predecessor (same for successor at line 15)
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:11`
```rust
            _ => 0xF,
```
**Suggested fix:** Accept only FenceArg (validated) and Imm(0); return Err for every other kind
```rust
            Operand::FenceArg(s) => parse_fence_bits(s)?,
            Operand::Imm(0) => 0,
            other => return Err(format!("invalid fence operand: {:?}", other)),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fence_neg_invalid_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: invalid pred Reg("x0") must Err (llvm-mc fence operand rule); got Ok(Word(254803983))
minimal failing input: bad = Reg("x0")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fence_pbt.rs
