# Bug: encode_adrp accepts W, SP, and FP/SIMD destinations
**Law:** ADRP Rd is Xd (X31=XZR, never SP); W registers and FP/SIMD registers are invalid and must return Err
**Impact:** `adrp w0, foo` / `adrp sp, foo` / `adrp d0, foo` encode as `adrp x0` / `adrp xzr` / `adrp x0`, so invalid GNU assembly is silently assembled into a different instruction
**Function:** encode_adrp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:653
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** encode_adrp([Reg("w0"), Symbol("s0")])
**Expected:** Err (llvm-mc and GNU as reject `adrp w0, foo`; ARM ADRP <Xd>)
**Actual:** Ok(WordWithReloc { word: 0x90000000, reloc_type: AdrpPage21, symbol: "s0", addend: 0 })
**Severity:** medium
**Root cause:** load_store.rs:654 discards get_reg's is_64 flag and never rejects SP (is_64=true, num=31) or FP prefixes that parse_reg_num accepts
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:654`
```rust
    let (rd, _) = get_reg(operands, 0)?;
```
**Suggested fix:** Require a 64-bit X register that is not SP before packing Rd
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let name = match operands.get(0) {
        Some(Operand::Reg(n)) => n.as_str(),
        _ => return Err("adrp needs Xd destination".into()),
    };
    let lower = name.to_ascii_lowercase();
    if !is_64 || lower == "sp" || matches!(lower.chars().next(), Some('d' | 's' | 'q' | 'v' | 'h' | 'b')) {
        return Err(format!("adrp destination must be Xd, got {name}"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_adrp_regression_w_reg -- --test-threads=1
```
**Raw output:**
```text
Test failed: ADRP takes Xd only; w0 must Err (gas/llvm-mc reject it) at src/backend/arm/assembler/encoder/encode_adrp_pbt.rs:448.
minimal failing input: kind = 0, n = 0, suffix = 0, fp_prefix = "d", use_wsp = false
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_adrp_pbt.rs
