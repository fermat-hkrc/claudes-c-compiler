# Bug: encode_tlbi rejects ARM default-CPU TLBI ops llvm-mc/gas assemble
**Law:** The assembler accepts the same textual assembly GCC's gas would consume for `tlbi`; ARM TLBI ops the default CPU accepts (alle2, alle3, alle3is, vae3, vae3is, vale3, vale3is) must encode
**Impact:** Valid GNU assembly `tlbi alle2` / `tlbi vae3is, x0` fails to assemble. The match table implements alle2is but not alle2, and omits all EL3 variants llvm-mc accepts without extra -mattr
**Function:** encode_tlbi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:477
**Detected by:** Differential vs llvm-mc -triple=aarch64 -show-encoding
**Minimal input:** encode_tlbi([], "vae3is, x0")
**Expected:** Ok(Word(0xd50e8320)) matching llvm-mc
**Actual:** Err("unsupported tlbi operation: vae3is")
**Severity:** low (documented by the author)
**Root cause:** system.rs:534 — the match table has no arms for alle2 / alle3 / alle3is / vae3 / vae3is / vale3 / vale3is, so those names fall through to the unsupported-op error
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:534`
```rust
        _ => return Err(format!("unsupported tlbi operation: {}", op_name)),
```
**Suggested fix:** Add the missing ARMv8.0 encodings (ALLE2 = SYS #4,C8,C7,#0; ALLE3IS = SYS #6,C8,C3,#0; VAE3IS = SYS #6,C8,C3,#1, Xt; …)
```rust
        "alle2"     => 0xd50c871f,
        "alle3is"   => 0xd50e831f,
        "alle3"     => 0xd50e871f,
        "vae3is"    => 0xd50e8320,
        "vae3"      => 0xd50e8720,
        "vale3is"   => 0xd50e83a0,
        "vale3"     => 0xd50e87a0,
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tlbi_regression_alle2 -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_tlbi_pbt::test_encode_tlbi_regression_alle2' panicked at src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs:389:31:
TLBI ALLE2 is a valid ARM op (llvm-mc/gas assemble tlbi alle2): "unsupported tlbi operation: alle2"
Test failed: ARM TLBI op "vae3is" must encode (llvm-mc accepts tlbi vae3is, x0): "unsupported tlbi operation: vae3is".
minimal failing input: kind = 1, i = 12, t = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs
