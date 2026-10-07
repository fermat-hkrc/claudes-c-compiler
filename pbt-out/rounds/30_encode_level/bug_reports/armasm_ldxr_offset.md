# encode_ldxr_stxr silently encodes non-zero imm9 for LDXR with offset (LDXR x0, [x1, #8] → imm9=479 instead of rejecting or encoding 0)
> **Filed as [issue #1075](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1075)**

# Bug: encode_ldxr_stxr silently encodes non-zero imm9 for LDXR with offset
**Law:** LDXR/STXR (load/store exclusive) support ONLY register-offset addressing [Xn] with NO immediate offset — the imm9 field must be 0. GNU as and llvm-mc reject `ldxr x0, [x1, #8]`.
**Impact:** `encode_ldxr_stxr([Reg("x0"), Mem{base:"x1",offset:8}], true, None)` returns Ok with imm9=479 (0x1DF) instead of Err. The encoded instruction loads from a wrong address (imm9=479 → signed -33 → address x1-33*8 or similar garbage).
**Function:** encode_ldxr_stxr (load_store.rs)
**Source location:** src/backend/arm/assembler/encoder/load_store.rs — LDXR path does not validate offset==0 before encoding
**Detected by:** round-30 adversarial encode PBT (p_ldxr_with_offset)
**Minimal input:** `encode_ldxr_stxr([Reg("x0"), Mem{base:"x1",offset:8}], true, None)` → Ok with imm9=479
**Expected:** Err("LDXR does not support immediate offset") or encode with imm9=0
**Actual:** Ok with imm9=479 (garbage offset)
**Severity:** medium (silent wrong address for exclusive load)
**Contract evidence:** inferred (ARM ARM: LDXR has no offset form; both refs reject)
**Fix:** In encode_ldxr_stxr, check that the Mem offset is 0 before encoding; return Err for non-zero offsets.

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="/home/shuhao/fermat-users/leo/.cargo/bin:/home/shuhao/fermat-users/leo/.pi-pbt/agent/bin:/home/shuhao/fermat-users/leo/.local/bin:/home/shuhao/fermat-users/leo/bin:/home/shuhao/fermat-users/leo/.bun/bin:/home/shuhao/fermat-users/leo/devecostudio/tools/emulator:/home/shuhao/fermat-users/leo/command-line-tools/bin:/home/shuhao/fermat-users/leo/.local/bin:/home/shuhao/fermat-users/leo/.local/bin:/home/shuhao/fermat-users/leo/bin:/home/shuhao/fermat-users/leo/.bun/bin:/home/shuhao/fermat-users/leo/devecostudio/tools/emulator:/home/shuhao/fermat-users/leo/command-line-tools/bin:/home/shuhao/fermat-users/leo/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin" cargo test --lib load_store::pbt_adversarial::p_ldxr_with_offset -- --nocapture
# assertion failed: LDXR with offset 8 encoded imm9=479
```
