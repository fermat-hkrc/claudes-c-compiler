# Bug: encode_vsetvli rejects RVV 1.0 SEW e128/e256/e512/e1024
**Law:** ∀ rd, rs1 ∈ GPR, sew ∈ {e128, e256, e512, e1024}, lmul ∈ {m1,m2,m4,m8,mf2,mf4,mf8}, ta ∈ {ta,tu}, ma ∈ {ma,mu}. encode_vsetvli([Reg(rd), Reg(rs1), Symbol(sew), Symbol(lmul), Symbol(ta), Symbol(ma)]) = Word(w) ∧ w = llvm-mc("vsetvli rd, rs1, sew, lmul, ta, ma")
**Impact:** Valid RVV 1.0 vsetvli with SEW wider than 64 cannot be assembled. Handwritten or compiler-emitted `vsetvli rd, rs1, e128, m1, tu, mu` (and e256/e512/e1024) fails with `unknown vtypei field` instead of the 32-bit word llvm-mc produces. The assembler claims the V (vector) standard extension.
**Function:** encode_vsetvli
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:48
**Detected by:** Differential — llvm-mc RVV vsetvli wide SEW
**Minimal input:** encode_vsetvli([Reg("x0"), Reg("x0"), Symbol("e128"), Symbol("m1"), Symbol("tu"), Symbol("mu")])
**Expected:** Ok(Word(0x02007057)) — llvm-mc `-triple=riscv64 -mattr=+v` encoding of `vsetvli x0, x0, e128, m1, tu, mu` (vsew=100)
**Actual:** Err("unknown vtypei field: e128")
**Severity:** medium
**Root cause:** vector.rs:21-38 parse_vtypei match lists only e8/e16/e32/e64; e128/e256/e512/e1024 fall through to the unknown-field error. No comment declares those SEW values invalid.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:22`
```rust
            "e8" => sew = 0b000,
            "e16" => sew = 0b001,
            "e32" => sew = 0b010,
            "e64" => sew = 0b011,
            "m1" => lmul = 0b000,
```
**Suggested fix:** Accept the remaining RVV 1.0 SEW names (vsew=100/101/110/111).
```rust
            "e8" => sew = 0b000,
            "e16" => sew = 0b001,
            "e32" => sew = 0b010,
            "e64" => sew = 0b011,
            "e128" => sew = 0b100,
            "e256" => sew = 0b101,
            "e512" => sew = 0b110,
            "e1024" => sew = 0b111,
            "m1" => lmul = 0b000,
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetvli_regression_wide_sew_e128 -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vsetvli_pbt::encode_vsetvli_diff_llvm_mc_wide_sew' panicked at src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs:287:1:
Test failed: SUT rejected valid vsetvli x0, x0, e128, m1, tu, mu: unknown vtypei field: e128.
minimal failing input: rd = "x0", rs1 = "x0", sew = "e128", lmul = "m1", ta = "tu", ma = "mu"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs
