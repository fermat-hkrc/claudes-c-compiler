# Bug: encode_vsetivli rejects RVV 1.0 wide SEW e128/e256/e512/e1024
**Law:** ∀ rd ∈ GPR names, uimm ∈ 0..=31, sew ∈ {e128,e256,e512,e1024}, lmul ∈ {m1,m2,m4,m8,mf2,mf4,mf8}, ta ∈ {ta,tu}, ma ∈ {ma,mu}. encode_vsetivli([Reg(rd), Imm(uimm), Symbol(sew), Symbol(lmul), Symbol(ta), Symbol(ma)]) = Word(w) ∧ w = llvm-mc("vsetivli rd, uimm, sew, lmul, ta, ma")
**Impact:** Valid RVV 1.0 vsetivli with SEW larger than e64 is rejected, so the assembler cannot emit the encodings llvm-mc produces (vsew=100/101/110/111). Any caller assembling `vsetivli ..., e128, ...` gets an assembler error instead of a 32-bit word.
**Function:** encode_vsetivli
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:59
**Detected by:** Differential — llvm-mc RISC-V assembler (RVV vsetivli)
**Minimal input:** encode_vsetivli([Reg("x0"), Imm(0), Symbol("e128"), Symbol("m1"), Symbol("tu"), Symbol("mu")])
**Expected:** Ok(Word(0xc2007057)) — llvm-mc `vsetivli x0, 0, e128, m1, tu, mu`
**Actual:** Err("unknown vtypei field: e128")
**Severity:** medium
**Root cause:** parse_vtypei (vector.rs:22-29) only matches e8/e16/e32/e64. Wide SEW names fall through to `unknown vtypei field`. encode_vsetivli calls parse_vtypei with no additional SEW handling.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:22`
```rust
            "e8" => sew = 0b000,
            "e16" => sew = 0b001,
            "e32" => sew = 0b010,
            "e64" => sew = 0b011,
```
**Suggested fix:** Accept the remaining vsew encodings that llvm-mc and RVV 1.0 name.
```rust
            "e8" => sew = 0b000,
            "e16" => sew = 0b001,
            "e32" => sew = 0b010,
            "e64" => sew = 0b011,
            "e128" => sew = 0b100,
            "e256" => sew = 0b101,
            "e512" => sew = 0b110,
            "e1024" => sew = 0b111,
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetivli_regression_wide_sew_e128 -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vsetivli_pbt::encode_vsetivli_diff_llvm_mc_wide_sew' panicked at src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs:298:1:
Test failed: SUT rejected valid vsetivli x0, 0, e128, m1, tu, mu: unknown vtypei field: e128.
minimal failing input: rd = "x0", uimm = 0, sew = "e128", lmul = "m1", ta = "tu", ma = "mu"
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs
