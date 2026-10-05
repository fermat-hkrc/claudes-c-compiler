# Bug: encode_neon_two_misc encodes SADDLP/UADDLP/SADALP/UADALP size from dest T, not source
**Law:** Pairwise-long two-misc (SADDLP/UADDLP/SADALP/UADALP) Vd.Ta, Vn.Tb must encode size from the SOURCE element size (Tb), with Q from the 64/128-bit choice (Ta)
**Impact:** Legal `saddlp v0.4h, v0.8b` is encoded as `saddlp v0.2s, v0.4h` (16-bit pairwise add instead of 8-bit). `saddlp v0.2d, v1.4s` is encoded with reserved size=11. Any compiler-emitted pairwise-long instruction is the wrong SIMD operation
**Function:** encode_neon_two_misc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1407
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** encode_neon_two_misc([v0.4h, v0.8b], u_bit=0, opcode=0b00010)
**Expected:** 0x0e202800 (llvm-mc `saddlp v0.4h, v0.8b`, size=00 from source 8b)
**Actual:** 0x0e602800 (size=01 from dest 4h — encoding of `saddlp v0.2s, v0.4h`)
**Severity:** high
**Root cause:** neon.rs:1408-1410 takes Q and size from the destination arrangement and discards the source arrangement, but ARM pairwise-long size is the source esize
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1408`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** For opcodes 00010/00110 derive size from source Tb and Q from dest Ta; require the ARM (Tb,Ta) pairing
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (q, size) = if opcode == 0b00010 || opcode == 0b00110 {
        match (arr_n.as_str(), arr_d.as_str()) {
            ("8b", "4h") => (0u32, 0b00u32),
            ("16b", "8h") => (1, 0b00),
            ("4h", "2s") => (0, 0b01),
            ("8h", "4s") => (1, 0b01),
            ("2s", "1d") => (0, 0b10),
            ("4s", "2d") => (1, 0b10),
            _ => return Err(format!("pairwise-long: expected (Tb,Ta) pair, got {}, {}", arr_n, arr_d)),
        }
    } else {
        if arr_d != arr_n {
            return Err(format!("two-misc: arrangement mismatch {} vs {}", arr_d, arr_n));
        }
        neon_arr_to_q_size(&arr_d)?
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_two_misc_regression_pairwise_size_from_dest -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_two_misc_pbt::encode_neon_two_misc_diff_llvm_mc_pairwise' (2372447) panicked at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:356:1:
Test failed: assertion failed: `(left == right)`
  left: `241182720`,
 right: `236988416`: SUT vs llvm-mc for saddlp v0.4h, v0.8b at src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs:384.
minimal failing input: rd = 0, rn = 0, mnem_case = (
    "saddlp",
    0,
    2,
), pair = (
    "8b",
    "4h",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs
