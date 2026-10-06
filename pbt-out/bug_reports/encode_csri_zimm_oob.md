# Bug: encode_csri masks out-of-range zimm instead of rejecting
**Law:** zimm must be in 0..=31 (RISC-V uimm5 / llvm-mc "immediate must be an integer in the range [0, 31]"); values outside that range must return Err, not a wrapped csrrwi/csrrsi/csrrci encoding.
**Impact:** `csrrwi rd, csr, 32` is assembled as `csrrwi rd, csr, 0` and `csrrwi rd, csr, -1` as `csrrwi rd, csr, 31`, silently writing the wrong immediate into a CSR.
**Function:** encode_csri
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:56
**Detected by:** Negative/Error Contract
**Minimal input:** encode_csri([Reg("x0"), Csr("fflags"), Imm(-1)], funct3=0b101)  // csrrwi x0, fflags, -1
**Expected:** Err
**Actual:** Ok(Word(2084979))  // 0x001FD073 = csrrwi x0, fflags, 31
**Severity:** high
**Root cause:** system.rs:59-60 casts the immediate to u32 and masks with 0x1F, so -1 becomes zimm 31 and 32 becomes zimm 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:59`
```rust
    let zimm = get_imm(operands, 2)? as u32;
    let rs1 = zimm & 0x1F;
```
**Suggested fix:** Reject zimm outside 0..=31 before masking.
```rust
    let zimm = get_imm(operands, 2)?;
    if !(0..=31).contains(&zimm) {
        return Err("csri: zimm out of range 0..=31".to_string());
    }
    let rs1 = (zimm as u32) & 0x1F;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_csri_neg_zimm_oob -- --test-threads=1
```
**Raw output:**
```text
Test failed: zimm -1 outside 0..=31 must Err (llvm-mc uimm5); got Ok(Word(2084979)) at src/backend/riscv/assembler/encoder/encode_csri_pbt.rs:501.
minimal failing input: (_mn, f3) = (
    "csrrwi",
    5,
), rd = "x0", (csr_name, _num) = (
    "fflags",
    1,
), zimm = -1
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_csri_pbt.rs
