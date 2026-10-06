# AArch64 parser accepts XZR/WSP/x31 as memory base addresses (ldr/ldp/str silently encode as SP-based loads)
> **Filed as [issue #828](https://github.com/fermat-hkrc/claudes-c-compiler/issues/828)**

# Bug: AArch64 parser accepts XZR/WSP/x31 as memory base addresses
**Law:** XZR (reg 31 in non-base context) and WSP are not valid memory base addresses. `ldr x0, [xzr]` means "load from address 0" via a register that doesn't hold addresses; `str x0, [wsp]` uses the 32-bit stack pointer name in a 64-bit base slot. Both are invalid; only SP (x31 in base context) is legal.
**Impact:** `ldr x0, [xzr]` encodes `e00340f9` = ldr x0, [sp] — XZR's reg-31 number is silently treated as SP in the base slot; `str x0, [wsp]` → `e00300f9` = str x0, [sp] — the 32-bit WSP name silently becomes SP. Both as and llvm-mc reject all six forms (`[xzr]`, `[x31]`, `[wsp]` in ldr/ldp/str).
**Function:** AArch64 assembler parser (base-register validation — #825 class at the addressing path)
**Detected by:** round-26j zero-register-in-base battery (14 forms, 6 BUGs)
**Minimal input:** `.text` / `ldr x0, [xzr]` → ccc-arm assembles `e00340f9` (ldr x0, [sp])
**Expected:** Err (xzr is not a base register)
**Actual:** silently encoded as SP-based load
**Severity:** medium
**Regression test:** (to add to pbt_asm_deep)

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
printf '.text\nldr x0, [xzr]\n' > /tmp/t.s
./target/release/ccc-arm -mlittle-endian -c -o /tmp/t.o /tmp/t.s && readelf -x .text /tmp/t.o  # e00340f9 = ldr x0,[sp]
aarch64-linux-gnu-as /tmp/t.s -o /dev/null  # Error
```

