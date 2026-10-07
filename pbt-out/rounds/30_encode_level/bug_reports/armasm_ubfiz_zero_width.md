# encode_ubfiz panics on subtraction overflow when width=0 (u32 underflow in imms computation)
> **Filed as [issue #1080](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1080)**

# Bug: encode_ubfiz panics on subtraction overflow when width=0
**Law:** encode_ubfiz returns Result — width=0 must produce Err, never panic. Same pattern as encode_shift #1073.
**Impact:** encode_ubfiz with width=0 panics with "attempt to subtract with overflow" — imms = lsb + width - 1 = 0 + 0 - 1 = -1 → u32 underflow.
**Function:** encode_ubfiz (bitfield.rs)
**Source location:** src/backend/arm/assembler/encoder/bitfield.rs — imms = lsb + width - 1 (u32 underflow)
**Detected by:** round-30 adversarial encode PBT (p_ubfiz_zero_width)
**Severity:** medium (crash in debug, silent wrong encoding in release)
**Contract evidence:** inferred (Result return type; width=0 invalid per ARM ARM)
**Fix:** Check width >= 1 before computing imms. Also check encode_sbfiz (same formula).

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="/home/shuhao/fermat-users/leo/.cargo/bin:/home/shuhao/fermat-users/leo/.pi-pbt/agent/bin:/home/shuhao/fermat-users/leo/.local/bin:/home/shuhao/fermat-users/leo/bin:/home/shuhao/fermat-users/leo/.bun/bin:/home/shuhao/fermat-users/leo/devecostudio/tools/emulator:/home/shuhao/fermat-users/leo/command-line-tools/bin:/home/shuhao/fermat-users/leo/.local/bin:/home/shuhao/fermat-users/leo/.local/bin:/home/shuhao/fermat-users/leo/bin:/home/shuhao/fermat-users/leo/.bun/bin:/home/shuhao/fermat-users/leo/devecostudio/tools/emulator:/home/shuhao/fermat-users/leo/command-line-tools/bin:/home/shuhao/fermat-users/leo/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin" cargo test --lib bitfield::pbt_adversarial2::p_ubfiz_zero_width -- --nocapture
```
