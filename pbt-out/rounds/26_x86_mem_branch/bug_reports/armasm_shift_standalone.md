# AArch64: shift amount ranges unvalidated on all standalone shift forms (lsl/lsr/asr/ror) + variable-shift mnemonics lslv/lsrv/asrv/rorv entirely missing
> **Filed as [issue #830](https://github.com/fermat-hkrc/claudes-c-compiler/issues/830)**

# Bug: AArch64 shift amount ranges unvalidated on all standalone shift forms; variable-shift mnemonics missing
**Law:** Standalone shift immediates are 0..63 (X) / 0..31 (W); negative values are invalid. The variable-shift aliases (lslv/lsrv/asrv/rorv) are standard AArch64 mnemonics that GNU as and llvm-mc both assemble.
**Impact:** `lsl x0, x1, #64` silently accepted (shift amount 64 is unencodable in the 6-bit imm field — masks to 0); `lsr w0, w1, #32` same (5-bit W field → 0); `asr x0, x1, #64`, `ror x0, x1, #64`, `ror w0, w1, #32`, `lsl x0, x1, #-1` — all 6 silently accepted while both refs reject. Additionally the register-operand forms `lslv x0, x1, x2`, `lsrv/asrv/rorv` are rejected entirely (both refs accept).
**Function:** AArch64 assembler shift encoding paths + mnemonic table
**Detected by:** round-26l 20-form shift battery (14 valid forms all byte-exact ✓; 6 BUG + 4 GAP = the two classes above; #793 already filed the imm12+shift and mov-hw variants — this extends it to the standalone shift forms)
**Minimal input:** `.text` / `lsl x0, x1, #64` → ccc-arm assembles (shift-by-0); both refs reject
**Expected:** Err for #64/#32/#-1; Ok for lslv/lsrv/asrv/rorv
**Actual:** BUG on all 6 range violations; GAP on all 4 variable-shift mnemonics
**Severity:** medium (silent wrong shift = the #776 class) + low (missing mnemonics = #818 class)
**Regression test:** (to add)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nlsl x0, x1, #64\n' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s  # accepts!
aarch64-linux-gnu-as /tmp/t.s -o /dev/null  # Error: shift amount out of range
```

