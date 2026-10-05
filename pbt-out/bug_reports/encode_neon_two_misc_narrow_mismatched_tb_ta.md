# Bug: encode_neon_two_misc_narrow accepts dest 8H with source 4S on XTN (Q=0)
**Law:** XTN{2}/SQXTN{2}/UQXTN{2}/SQXTUN{2} are defined only for Ta in {8H,4S,2D} with matching Tb {8B/16B, 4H/8H, 2S/4S} selected by Q; every other pair must be Err
**Impact:** Invalid assembly such as `xtn v0.8h, v0.4s` encodes as XTN Vd.4H, Vn.4S (size from source only, Q from is_high), producing the wrong machine code instead of an assemble error. A mistyped dest arrangement is silently rewritten.
**Function:** encode_neon_two_misc_narrow
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:206
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_two_misc_narrow([v0.8h, v0.4s], u_bit=0, opcode=0b10010, is_high=false)
**Expected:** Err (llvm-mc: invalid operand; ARM XTN Vn.4S requires Vd.4H; 8H is XTN2)
**Actual:** Ok(Word(0x0e612800)) — dest "8h" is discarded; source "4s" sets size=01 and is_high=false sets Q=0
**Severity:** high
**Root cause:** neon.rs:210 binds dest arrangement as `_arr_d` and never checks it against Ta or is_high; size is taken only from the source arrangement
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:210`
```rust
    let (rd, _arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require the ARM-mandated dest arrangement for (Ta, is_high)
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let expected_tb = match (arr_n.as_str(), is_high) {
        ("8h", false) => "8b",
        ("8h", true) => "16b",
        ("4s", false) => "4h",
        ("4s", true) => "8h",
        ("2d", false) => "2s",
        ("2d", true) => "4s",
        _ => return Err(format!("unsupported source arrangement for narrow: {}", arr_n)),
    };
    if arr_d != expected_tb {
        return Err(format!("narrow: source {} requires dest {}", arr_n, expected_tb));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_two_misc_narrow_neg_mismatched_tb_ta -- --test-threads=1
cargo test --lib test_encode_neon_two_misc_narrow_regression_mismatched_tb_ta -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_two_misc_narrow_pbt::encode_neon_two_misc_narrow_neg_mismatched_tb_ta' (2513885) panicked at src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs:452:1:
Test failed: invalid/mismatched Tb/Ta must Err (ARM XTN Ta in {8H,4S,2D} with matching Tb; llvm-mc rejects xtn v0.8h, v0.4s) at src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs:473.
minimal failing input: rd = 0, rn = 0, tb = "8h", ta = "4s", is_high = false, fam = (
    "xtn",
    0,
    18,
)
	successes: 3
	local rejects: 0
	global rejects: 1
		1 times at src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs:464:9: !is_valid_pair(tb, ta, is_high)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_two_misc_narrow_pbt.rs
