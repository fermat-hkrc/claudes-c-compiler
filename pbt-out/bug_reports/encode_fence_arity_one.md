# Bug: encode_fence with one operand encodes fence iorw, iorw
**Law:** A single fence operand must be rejected; llvm-mc reports `too few operands for instruction` for `fence iorw` and `fence 0`
**Impact:** `fence rw` (one operand) silently becomes a full `fence iorw, iorw` rather than an error or `fence rw, rw`, so a truncated line is assembled as a stronger barrier
**Function:** encode_fence
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:5
**Detected by:** Negative/Error Contract
**Minimal input:** encode_fence([FenceArg("iorw")])
**Expected:** Err
**Actual:** Ok(Word(0x0ff0000f))
**Severity:** medium
**Root cause:** system.rs:18-19 — the else branch (len == 1) hard-codes (0xF, 0xF), the same default as the documented empty-operand case, and always returns Ok
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:18`
```rust
    } else {
        (0xF, 0xF)
    };
```
**Suggested fix:** Reject arity 1; keep the empty default only for no operands
```rust
    } else {
        return Err("fence requires 0 or 2 operands".into());
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fence_neg_arity -- --test-threads=1
```
**Raw output:**
```text
Test failed: single operand must Err (llvm-mc too few operands); got Ok(Word(267386895))
minimal failing input: op = FenceArg("iorw")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_fence_pbt.rs
