# encode_ldaxr_stlxr silently encodes non-zero imm9 for LDAXR with offset (LDAXR x0, [x1, #8] accepted) — third instance of the exclusive-load offset pattern
> **Filed as [issue #1078](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1078)**

# Bug: encode_ldaxr_stlxr silently encodes non-zero imm9 for LDAXR with offset
**Law:** LDAXR (load-acquire exclusive) supports ONLY [Xn] addressing with NO immediate offset — imm9 must be 0. Both refs reject.
**Impact:** `encode_ldaxr_stlxr([Reg("x0"), Mem{base:"x1",offset:8}], true, None)` returns Ok instead of Err. Third instance of the exclusive-load offset pattern: LDXR (#1075, imm9=479), LDAR (#1077, imm9=511), now LDAXR. Systemic: load-exclusive/acquire functions don't validate offset.
**Function:** encode_ldaxr_stlxr (load_store.rs)
**Detected by:** round-30 adversarial encode PBT offset sweep (p_all_offset_validation)
**Severity:** medium (silent wrong address for acquire-exclusive load)
**Contract evidence:** inferred (ARM ARM: LDAXR has no offset form)
**Fix:** Check offset==0 in encode_ldaxr_stlxr; return Err for non-zero.

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="/home/shuhao/fermat-users/leo/.cargo/bin:/home/shuhao/fermat-users/leo/.pi-pbt/agent/bin:/home/shuhao/fermat-users/leo/.local/bin:/home/shuhao/fermat-users/leo/bin:/home/shuhao/fermat-users/leo/.bun/bin:/home/shuhao/fermat-users/leo/devecostudio/tools/emulator:/home/shuhao/fermat-users/leo/command-line-tools/bin:/home/shuhao/fermat-users/leo/.local/bin:/home/shuhao/fermat-users/leo/.local/bin:/home/shuhao/fermat-users/leo/bin:/home/shuhao/fermat-users/leo/.bun/bin:/home/shuhao/fermat-users/leo/devecostudio/tools/emulator:/home/shuhao/fermat-users/leo/command-line-tools/bin:/home/shuhao/fermat-users/leo/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin" cargo test --lib load_store::pbt_offset_sweep -- --nocapture
# LDAXR: ACCEPTED (potential BUG)
```
