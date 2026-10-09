# Bug: unsuffixed `mov` between mismatched-width GP registers silently encodes using first-register size

**Law:** Unsuffixed `mov` between two GP registers is valid only when both have the same width; llvm-mc/GAS reject `mov %ax, %al` / `mov %eax, %ax` with "unknown use of instruction mnemonic without a size suffix". The encoder must not pick the first register's size and emit a one-sided encoding.
**Impact:** Size-mismatched register moves in unsuffixed inline asm produce wrong opcodes/prefixes (e.g. 16-bit form with 0x66 for `mov %ax, %al`) instead of a hard error, so assemblers accept code other tools reject and emit incorrect machine code.
**Function:** encode_mov_infer_size
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:112
**Detected by:** Differential — accept/reject vs llvm-mc i686 (mismatched-width contract)
**Minimal input:** `mov %ax, %al` → ops `[Reg(ax), Reg(al)]`
**Expected:** `Err(...)` (mismatched widths; require explicit suffix / movzx/movsx)
**Actual:** `Ok([0x66, 0x89, 0xc0])` — first-reg size=2 applied; encodes as 16-bit mov to al's reg number
**Severity:** high
**Root cause:** `gp_integer.rs:118` always takes `reg_size` of the *first* Register operand and never compares it to the second register's width before calling `encode_mov`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:117`
```rust
        let size = match (&ops[0], &ops[1]) {
            (Operand::Register(r), _) => reg_size(&r.name),
            (_, Operand::Register(r)) => reg_size(&r.name),
            _ => 4, // default to 32-bit
        };
        self.encode_mov(ops, size)
```
**Suggested fix:** When both operands are GP registers, require equal `reg_size` (skip CR/Sreg pairs which take specialized paths inside `encode_mov`):
```rust
        if let (Operand::Register(a), Operand::Register(b)) = (&ops[0], &ops[1]) {
            let sa = reg_size(&a.name);
            let sb = reg_size(&b.name);
            // CR/Sreg handled inside encode_mov; only gate GP↔GP width match
            if !is_control_reg(&a.name)
                && !is_control_reg(&b.name)
                && !is_segment_reg(&a.name)
                && !is_segment_reg(&b.name)
                && sa != sb
            {
                return Err(format!(
                    "mov operand size mismatch: {} ({}-byte) vs {} ({}-byte)",
                    a.name, sa, b.name, sb
                ));
            }
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_infer_size_regression_mismatched_width -- --test-threads=1
```
**Raw output:**
```text
thread '...' panicked at .../encode_mov_infer_size_pbt.rs:
mismatched `mov %ax, %al` must Err; got Ok(Some([66, 89, c0]))
minimal failing input: w1 = 2, w2 = 1, si = 0, di = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_infer_size_pbt.rs
