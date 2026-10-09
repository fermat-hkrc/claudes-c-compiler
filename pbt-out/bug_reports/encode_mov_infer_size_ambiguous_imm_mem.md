# Bug: unsuffixed `mov $imm, mem` silently defaults to 32-bit instead of rejecting

**Law:** Unsuffixed `mov` must infer operand size from register operands; when neither operand is a register (imm→mem), GAS/llvm-mc reject the instruction as ambiguous and require an explicit `movb`/`movw`/`movl` suffix. The encoder must not invent a size.
**Impact:** Inline asm / assembler paths that omit a size suffix on store-immediate emit wrong-width stores (always dword), corrupting adjacent memory or generating encodings that other tools reject as ambiguous.
**Function:** encode_mov_infer_size
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:112
**Detected by:** Differential — accept/reject vs llvm-mc i686 (ambiguous-suffix contract)
**Minimal input:** `mov $0, (%eax)` → ops `[Imm(0), Mem(base=eax)]`
**Expected:** `Err(...)` (ambiguous; no register to infer size)
**Actual:** `Ok([0xc7, 0x00, 0x00, 0x00, 0x00, 0x00])` — silent `movl $0, (%eax)`
**Severity:** high
**Root cause:** `gp_integer.rs:121` `_ => 4` defaults size to 32-bit when neither operand is a Register, then `encode_mov` happily emits C7 /0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:117`
```rust
        let size = match (&ops[0], &ops[1]) {
            (Operand::Register(r), _) => reg_size(&r.name),
            (_, Operand::Register(r)) => reg_size(&r.name),
            _ => 4, // default to 32-bit
        };
        self.encode_mov(ops, size)
```
**Suggested fix:** Reject when size cannot be inferred from a register operand (and optionally when both registers disagree on width):
```rust
        let size = match (&ops[0], &ops[1]) {
            (Operand::Register(r), _) => reg_size(&r.name),
            (_, Operand::Register(r)) => reg_size(&r.name),
            _ => {
                return Err(
                    "ambiguous mov: no register operand to infer size (use movb/movw/movl)"
                        .to_string(),
                );
            }
        };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_infer_size_regression_ambiguous_imm_mem -- --test-threads=1
```
**Raw output:**
```text
thread '...' panicked at .../encode_mov_infer_size_pbt.rs:
ambiguous `mov $0, (%eax)` must Err; got Ok(Some([c7, 00, 00, 00, 00, 00]))
minimal failing input: bi = 0, imm = 0, disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_infer_size_pbt.rs
