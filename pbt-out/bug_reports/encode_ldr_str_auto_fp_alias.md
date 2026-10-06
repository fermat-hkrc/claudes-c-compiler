# Bug: encode_ldr_str_auto rejects the GNU fp alias of X29
**Law:** `ldr`/`str fp, [Xn]` must encode as LDR/STR of X29, matching llvm-mc/gas
**Impact:** GNU-style assembly that uses `fp` (frame pointer, X29) as an LDR/STR data register is rejected with "invalid register: fp". The built-in assembler then fails on gas-accepted input (README.md:12).
**Function:** encode_ldr_str_auto
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:7
**Detected by:** Differential vs llvm-mc -triple=aarch64 -show-encoding (sweep)
**Minimal input:** encode_ldr_str_auto([Reg("fp"), Mem{base:"x1", offset:0}], is_load=true)
**Expected:** Word(0xf940003d) — llvm-mc `ldr fp, [x1]` == `ldr x29, [x1]`
**Actual:** Err("invalid register: fp")
**Severity:** medium
**Root cause:** encode_ldr_str_auto would treat `fp` as default 64-bit GP (first char `f` is not an FP prefix in `is_fp_reg`), but `parse_reg_num` (called via `get_reg` in `encode_ldr_str`) has no `fp => 29` alias, so the encode fails before a word is produced.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/mod.rs:294`
```rust
        "sp" | "wsp" => Some(31),
        "xzr" | "wzr" => Some(31),
        "lr" => Some(30),
        _ => {
```
**Suggested fix:** Alias `fp` to register 29 next to `lr` => 30.
```rust
        "sp" | "wsp" => Some(31),
        "xzr" | "wzr" => Some(31),
        "lr" => Some(30),
        "fp" => Some(29),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_auto_regression_fp_alias -- --test-threads=1
cargo test --lib encode_ldr_str_auto_diff_fp_alias -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT rejected valid str fp, [x0]: invalid register: fp.
minimal failing input: is_load = false, rn = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs
