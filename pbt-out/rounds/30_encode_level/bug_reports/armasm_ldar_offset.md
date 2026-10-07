# encode_ldar_stlr silently encodes non-zero imm9 for LDAR/STLR with offset (LDAR x0, [x1, #8] → imm9=511) — same pattern as LDXR #1075
> **Filed as [issue #1077](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1077)**

# Bug: encode_ldar_stlr silently encodes non-zero imm9 for LDAR/STLR with offset
**Law:** LDAR/STLR (load-acquire/store-release) support ONLY register-offset addressing [Xn] with NO immediate offset — the imm9 field must be 0. Both refs reject `ldar x0, [x1, #8]`.
**Impact:** `encode_ldar_stlr([Reg("x0"), Mem{base:"x1",offset:8}], true, None)` returns Ok with imm9=511 (all 1s) instead of Err. Same pattern as LDXR #1075 (imm9=479) but in a separate function (encode_ldar_stlr vs encode_ldxr_stxr). The encoded instruction accesses a wrong address.
**Function:** encode_ldar_stlr (load_store.rs)
**Source location:** src/backend/arm/assembler/encoder/load_store.rs — LDAR path does not validate offset==0
**Detected by:** round-30 adversarial encode PBT (p_ldar_with_offset)
**Minimal input:** `encode_ldar_stlr([Reg("x0"), Mem{base:"x1",offset:8}], true, None)` → Ok with imm9=511
**Expected:** Err("LDAR does not support immediate offset") or encode with imm9=0
**Actual:** Ok with imm9=511 (all ones — wrong address)
**Severity:** medium (silent wrong address for acquire/release)
**Contract evidence:** inferred (ARM ARM: LDAR/STLR has no offset form; both refs reject)
**Fix:** In encode_ldar_stlr, check that the Mem offset is 0 before encoding; return Err for non-zero offsets.

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="/home/shuhao/fermat-users/leo/.cargo/bin:/home/shuhao/fermat-users/leo/.pi-pbt/agent/bin:/home/shuhao/fermat-users/leo/.local/bin:/home/shuhao/fermat-users/leo/bin:/home/shuhao/fermat-users/leo/.bun/bin:/home/shuhao/fermat-users/leo/devecostudio/tools/emulator:/home/shuhao/fermat-users/leo/command-line-tools/bin:/home/shuhao/fermat-users/leo/.local/bin:/home/shuhao/fermat-users/leo/.local/bin:/home/shuhao/fermat-users/leo/bin:/home/shuhao/fermat-users/leo/.bun/bin:/home/shuhao/fermat-users/leo/devecostudio/tools/emulator:/home/shuhao/fermat-users/leo/command-line-tools/bin:/home/shuhao/fermat-users/leo/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin" cargo test --lib load_store::pbt_adversarial2::p_ldar_with_offset -- --nocapture
# assertion failed: LDAR with offset 8 encoded imm9=511
```
