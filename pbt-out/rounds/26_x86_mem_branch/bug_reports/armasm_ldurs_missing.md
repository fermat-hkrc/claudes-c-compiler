# AArch64 assembler lacks ldursb/ldursh/ldursw (unscaled sign-extending loads)
> **Filed as [issue #842](https://github.com/fermat-hkrc/claudes-c-compiler/issues/842)**

# Bug: AArch64 assembler lacks ldursb/ldursh/ldursw unscaled sign-extending loads
**Law:** ldursb/ldursh/ldursw are the unscaled-offset variants of ldrsb/ldrsh/ldrsw (imm9 range -256..255 instead of scaled imm12); GNU as and llvm-mc assemble them.
**Impact:** `ldursb x0, [x1, #-256]` etc. rejected; the scaled variants (ldrsb with positive offsets) work. 3 missing mnemonics.
**Function:** AArch64 assembler load encoder (unscaled sign-extend path)
**Detected by:** round-26aa 21-form sign-ext battery (18 byte-exact ✓; 2 range BUGs noted on #792; 3 GAPs here)
**Minimal input:** `.text` / `ldursw x0, [x1, #-256]` → ccc rejects; both refs accept
**Expected:** assembles (B9 8000 80 encoded unscaled sign-ext word load)
**Actual:** rejected
**Severity:** low (completeness)
**Regression test:** (to add)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nldursw x0, [x1, #-256]\n' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s  # error
aarch64-linux-gnu-as /tmp/t.s -o /dev/null  # OK
```

