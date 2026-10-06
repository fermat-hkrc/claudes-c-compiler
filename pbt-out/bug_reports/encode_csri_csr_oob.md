# Bug: encode_csri masks out-of-range CSR numbers instead of rejecting
**Law:** csr must be in 0..=4095 (RISC-V csr[11:0] / llvm-mc "immediate must be an integer in the range [0, 4095]"); values outside that range must return Err, not a wrapped encoding.
**Impact:** `csrrwi rd, 4096, zimm` is assembled as `csrrwi rd, 0, zimm` and `csrrwi rd, -1, zimm` as `csrrwi rd, 4095, zimm`, silently targeting the wrong CSR.
**Function:** encode_csri
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:56
**Detected by:** Negative/Error Contract
**Minimal input:** encode_csri([Reg("x0"), Imm(-1), Imm(0)], funct3=0b101)  // csrrwi x0, -1, 0
**Expected:** Err
**Actual:** Ok(Word(4293939315))  // 0xFFF05073 = csrrwi x0, 4095, 0
**Severity:** high
**Root cause:** system.rs:58 takes csr via get_csr_num (Imm is `*v as u32` with no range check) and system.rs:61 passes `csr as i32` into encode_i, which masks with 0xFFF, so -1 becomes csr 4095 and 4096 becomes csr 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:61`
```rust
    Ok(EncodeResult::Word(encode_i(OP_SYSTEM, rd, funct3, rs1, csr as i32)))
```
**Suggested fix:** Reject csr outside 0..=4095 before packing.
```rust
    let csr = get_csr_num(operands, 1)?;
    if csr > 4095 {
        return Err("csri: csr out of range 0..=4095".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_csri_neg_csr_oob -- --test-threads=1
```
**Raw output:**
```text
Test failed: csr -1 outside 0..=4095 must Err (llvm-mc csr[11:0]); got Ok(Word(4293939315)) at src/backend/riscv/assembler/encoder/encode_csri_pbt.rs:518.
minimal failing input: (_mn, f3) = (
    "csrrwi",
    5,
), rd = "x0", zimm = 0, csr_num = -1
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_csri_pbt.rs
