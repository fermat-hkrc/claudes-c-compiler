# encode_shift panics on subtraction overflow when shift amount >= register width (lsl w0, w1, #32 → "attempt to subtract with overflow")
> **Filed as [issue #1073](https://github.com/fermat-hkrc/claudes-c-compiler/issues/1073)**

# Bug: encode_shift panics on subtraction overflow
**Law:** encode_shift returns Result<EncodeResult, String> — out-of-range inputs must produce Err, never panic. For W registers, shift amount must be 0..31; 32 is out of range.
**Impact:** `encode_shift([Reg("w0"), Reg("w1"), Imm(32)], 0)` panics with "attempt to subtract with overflow" in debug mode. In release mode it silently computes a huge imms value, producing a wrong instruction. The panic is at `imms = width - 1 - imm` where `width=32, imm=32` → `31 - 32` → u32 underflow.
**Function:** encode_shift (data_processing.rs, LSL immediate path)
**Source location:** src/backend/arm/assembler/encoder/data_processing.rs:811 — `let imms = width - 1 - imm;` (u32 underflow when imm >= width)
**Detected by:** round-30 adversarial encode PBT (p_shift_64_on_w)
**Minimal input:** `encode_shift([Reg("w0"), Reg("w1"), Imm(32)], 0)` → panics
**Expected:** Err("shift amount out of range") or equivalent
**Actual:** debug panic "attempt to subtract with overflow"
**Severity:** medium (crash in debug, silent wrong encoding in release)
**Contract evidence:** inferred (Result return type is the error contract)
**Fix:** Check `imm < width` before computing `imms = width - 1 - imm`, return Err for out-of-range shift. Also check LSR/ASR paths (same `width - 1` calculation without imm check).

## Found by

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="/home/shuhao/fermat-users/leo/.cargo/bin:/home/shuhao/fermat-users/leo/.pi-pbt/agent/bin:/home/shuhao/fermat-users/leo/.local/bin:/home/shuhao/fermat-users/leo/bin:/home/shuhao/fermat-users/leo/.bun/bin:/home/shuhao/fermat-users/leo/devecostudio/tools/emulator:/home/shuhao/fermat-users/leo/command-line-tools/bin:/home/shuhao/fermat-users/leo/.local/bin:/home/shuhao/fermat-users/leo/.local/bin:/home/shuhao/fermat-users/leo/bin:/home/shuhao/fermat-users/leo/.bun/bin:/home/shuhao/fermat-users/leo/devecostudio/tools/emulator:/home/shuhao/fermat-users/leo/command-line-tools/bin:/home/shuhao/fermat-users/leo/.local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/snap/bin" cargo test --lib data_processing::pbt_adversarial2::p_shift_64_on_w -- --nocapture
# thread panicked: attempt to subtract with overflow
```
