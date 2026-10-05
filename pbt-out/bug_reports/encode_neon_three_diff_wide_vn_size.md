# Bug: encode_neon_three_diff takes WIDE size/Q from Vn instead of narrow Vm
**Law:** For WIDE three-different instructions (SADDW/UADDW/SSUBW/USUBW), encode_neon_three_diff([Vd.Ta, Vn.Ta, Vm.Tb], U, opc, is_high) must equal llvm-mc("mnem{2} Vd.Ta, Vn.Ta, Vm.Tb") — size and Q come from the narrow source Vm.Tb
**Impact:** Valid SADDW/UADDW/SSUBW/USUBW (and *2) assemble to the wrong 32-bit word: Q and size are taken from the wide Vn arrangement, so `saddw v0.8h, v0.8h, v0.8b` is emitted as if it were a Q=1 size=01 encoding. Callers of the GNU-style assembler get silently wrong machine code for every WIDE form. `saddw v0.2d, v0.2d, v0.2s` is rejected outright because `2d` is not in the Vn match.
**Function:** encode_neon_three_diff
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:89
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** encode_neon_three_diff([v0.8h, v0.8h, v0.8b], u_bit=0, opcode=0b0001, is_high=false)  (`saddw v0.8h, v0.8h, v0.8b`)
**Expected:** 0x0e201000 (llvm-mc; Q=0, size=00 from Vm.8b)
**Actual:** 0x4e601000 (Q=1, size=01 from Vn.8h)
**Severity:** high
**Root cause:** neon.rs:98 matches `arr_n` (Vn). For LONG, Vn is narrow; for WIDE, Vn is wide (8h/4s/2d), so size/Q are wrong. The comment on that match says the narrow source should drive size.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:98`
```rust
    let (q, size) = match arr_n.as_str() {
```
**Suggested fix:** For WIDE opcodes (0001/0011) take size/Q from Vm (operand 2); for LONG keep Vn. Also accept `2d` as Vn when Vm supplies the narrow size.
```rust
    let arr_narrow = if opcode == 0b0001 || opcode == 0b0011 {
        &_arr_m
    } else {
        &arr_n
    };
    let (q, size) = match arr_narrow.as_str() {
        "8b" => (0u32, 0b00u32),
        "16b" => (1, 0b00),
        "4h" => (0, 0b01),
        "8h" => (1, 0b01),
        "2s" => (0, 0b10),
        "4s" => (1, 0b10),
        _ => return Err(format!("unsupported source arrangement for three-diff: {}", arr_narrow)),
    };
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_three_diff_diff_llvm_mc_wide -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `1314918400`,
 right: `236982272`: mismatch for saddw v0.8h, v0.8h, v0.8b
minimal failing input: rd = 0, rn = 0, rm = 0, pair = ("8h", "8b", false), insn = (0, 1, "saddw")
cc b6cdc5885ea4f946c082c808606b2f96b160a5285dd4918b6f30ec1201dc2afd
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs
