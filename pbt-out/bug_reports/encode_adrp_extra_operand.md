# Bug: encode_adrp ignores extra operands
**Law:** ADRP takes exactly two operands (Xd, symbol); extra operands must return Err as GNU as and llvm-mc do
**Impact:** `adrp x0, foo, x1` is assembled as `adrp x0, foo`, dropping the extra operand instead of diagnosing invalid assembly
**Function:** encode_adrp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:653
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** encode_adrp([Reg("x0"), Symbol("foo"), Reg("x0")])
**Expected:** Err (gas: "unexpected characters following instruction"; llvm-mc: "invalid operand")
**Actual:** Ok(WordWithReloc { word: 0x90000000, reloc_type: AdrpPage21, symbol: "foo", addend: 0 })
**Severity:** medium
**Root cause:** load_store.rs:653–690 never checks operands.len(); only operands[0] and operands[1] are read, so trailing operands are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:653`
```rust
pub(crate) fn encode_adrp(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, _) = get_reg(operands, 0)?;
```
**Suggested fix:** Reject anything other than exactly two operands
```rust
    if operands.len() != 2 {
        return Err(format!("adrp takes 2 operands, got {}", operands.len()));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_adrp_regression_extra_operand -- --test-threads=1
```
**Raw output:**
```text
Test failed: invalid ADRP arity/kind=2 must Err, got Ok(WordWithReloc { word: 2415919104, reloc: Relocation { reloc_type: AdrpPage21, symbol: "foo", addend: 0 } }) at src/backend/arm/assembler/encoder/encode_adrp_pbt.rs:496.
minimal failing input: kind = 2, rd = 0, extra = Reg("x0"), imm = 0
	successes: 4
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_adrp_pbt.rs
