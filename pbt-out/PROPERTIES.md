# Properties: encode_vsetvli

## encode_vsetvli_diff_llvm_mc_named
- Tier: 4
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent RISC-V assembler, same job: assemble `vsetvli` to a 32-bit word). State machine rejected — pure function, no lifecycle. Algebraic round-trip rejected — no in-tree vsetvli decoder. encode_vsetivli / encode_vsetvl rejected as siblings — same-job gate fails (different instruction, different bit31/rs1-vs-uimm layout). Wrapper encode_instruction passes operands through (mod.rs:941).
- Doc contract: vector.rs:46-47 "Encode vsetvli rd, rs1, vtypei" / "Format: [0][vtypei[10:0]][rs1][111][rd][1010111]" — asserted fingerprint 98424e07
- Seed: (none) — no project-owned vsetvli unit test
- Formal: ∀ rd, rs1 ∈ {x0..x31} ∪ ABI ∪ {fp}, sew ∈ {e8,e16,e32,e64}, lmul ∈ {m1,m2,m4,m8,mf2,mf4,mf8}, ta ∈ {ta,tu}, ma ∈ {ma,mu}. encode_vsetvli([Reg(rd),Reg(rs1),Symbol(sew),Symbol(lmul),Symbol(ta),Symbol(ma)]) = Word(w) ∧ w = llvm-mc("vsetvli rd, rs1, sew, lmul, ta, ma", triple=riscv64, mattr=+v)
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetvli
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs1, sew, lmul, ta, ma]
  domain: { rd: gpr_name, rs1: gpr_name, sew: {e8,e16,e32,e64}, lmul: {m1,m2,m4,m8,mf2,mf4,mf8}, ta: {ta,tu}, ma: {ma,mu} }
  relation:
    op: eq
    lhs: sut_word([Reg(rd), Reg(rs1), Symbol(sew), Symbol(lmul), Symbol(ta), Symbol(ma)])
    rhs: llvm_mc_word("vsetvli {rd}, {rs1}, {sew}, {lmul}, {ta}, {ma}")
generators:
  rd: { gen: string }
  rs1: { gen: string }
  sew: { gen: string }
  lmul: { gen: string }
  ta: { gen: string }
  ma: { gen: string }
evidence: vector.rs:46-47; encoder/mod.rs:941; README.md:14; llvm-mc -triple=riscv64 -mattr=+v
```

## encode_vsetvli_diff_llvm_mc_imm
- Tier: 4
- Rationale: llvm-mc accepts a raw 11-bit vtypei immediate 0..=2047 as the third operand (prints the decoded named form). parse_vtypei vector.rs:17-18 "Raw immediate: treat as pre-encoded vtypei value". Same differential reference as p1. Domain pinned to the documented 11-bit field (vtypei[10:0] in the rustdoc format line) including bounds 0 and 2047.
- Doc contract: vector.rs:46-47 "Format: [0][vtypei[10:0]][rs1][111][rd][1010111]" — asserted fingerprint 98424e07
- Seed: (none)
- Formal: ∀ rd, rs1 ∈ GPR names, v ∈ 0..=2047. encode_vsetvli([Reg(rd),Reg(rs1),Imm(v)]) = Word(w) ∧ w = llvm-mc("vsetvli rd, rs1, v")
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetvli
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs1, v]
  domain: { rd: gpr_name, rs1: gpr_name, v: 0..=2047 }
  relation:
    op: eq
    lhs: sut_word([Reg(rd), Reg(rs1), Imm(v)])
    rhs: llvm_mc_word("vsetvli {rd}, {rs1}, {v}")
generators:
  rd: { gen: string }
  rs1: { gen: string }
  v: { gen: int, min: 0, max: 2047, type: u32 }
evidence: vector.rs:17-18; vector.rs:46-47; llvm-mc accepts 0..=2047
```

## encode_vsetvli_format_fields
- Tier: 3
- Rationale: Algebraic invariant from the rustdoc format line and RISC-V V 1.0: opcode=1010111, funct3=111, bit31=0, rd in [11:7], rs1 in [19:15], vtypei in [30:20] packed as [ma][ta][sew][lmul]. Weaker than differential; kept as an llvm-mc-independent structural check. Round-trip rejected (no decoder).
- Doc contract: vector.rs:46-47 "Format: [0][vtypei[10:0]][rs1][111][rd][1010111]" — asserted fingerprint 98424e07
- Seed: (none)
- Formal: ∀ rd,rs1 ∈ 0..31, sew ∈ {0,1,2,3}, lmul ∈ {0,1,2,3,5,6,7}, ta,ma ∈ {0,1}. let w = encode_vsetvli(named). w[6:0]=1010111 ∧ w[14:12]=111 ∧ w[31]=0 ∧ w[11:7]=rd ∧ w[19:15]=rs1 ∧ w[30:20]=(ma<<7)|(ta<<6)|(sew<<3)|lmul
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetvli
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, sew, lmul, ta, ma]
  domain: { rd: 0..31, rs1: 0..31, sew: {0,1,2,3}, lmul: {0,1,2,3,5,6,7}, ta: 0..1, ma: 0..1 }
  relation:
    op: holds
    lhs: unpack_vsetvli(sut_word(named(rd,rs1,sew,lmul,ta,ma)))
    rhs: opcode==0b1010111 && funct3==0b111 && bit31==0 && rd_f==rd && rs1_f==rs1 && vtypei==(ma<<7)|(ta<<6)|(sew<<3)|lmul
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  sew: { gen: int, min: 0, max: 3, type: u32 }
  lmul: { gen: int, min: 0, max: 7, type: u32 }
  ta: { gen: int, min: 0, max: 1, type: u32 }
  ma: { gen: int, min: 0, max: 1, type: u32 }
evidence: vector.rs:46-47; vector.rs:42; RISC-V V 1.0 vsetvli
```

## encode_vsetvli_abi_xn_alias
- Tier: 3
- Rationale: Algebraic metamorphic: ABI names (zero/ra/sp/…/fp) must encode the same word as xN. Evidence: encoder/mod.rs:224-270 reg_num maps ABI and xN to the same 5-bit encoding; llvm-mc prints ABI names for xN. Not a differential (both sides are SUT).
- Doc contract: vector.rs:46 "Encode vsetvli rd, rs1, vtypei" — asserted fingerprint 41263020
- Seed: (none)
- Formal: ∀ n,m ∈ 0..31, vtype named. encode_vsetvli(ABI(n), ABI(m), vtype) = encode_vsetvli(xN(n), xN(m), vtype); n=8 also via fp
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetvli
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, sew, lmul, ta, ma]
  domain: { n: 0..31, m: 0..31, sew: named_sew, lmul: named_lmul, ta: {ta,tu}, ma: {ma,mu} }
  relation:
    op: eq
    lhs: sut_word(abi_ops(n, m, sew, lmul, ta, ma))
    rhs: sut_word(xn_ops(n, m, sew, lmul, ta, ma))
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  sew: { gen: string }
  lmul: { gen: string }
  ta: { gen: string }
  ma: { gen: string }
evidence: encoder/mod.rs:224-270 reg_num ABI/xN/fp
```

## encode_vsetvli_field_isolation
- Tier: 3
- Rationale: Algebraic metamorphic / isolation: changing rd must not alter opcode/funct3/rs1/vtypei/bit31; changing rs1 must not alter rd/opcode/vtypei; changing vtypei must not alter rd/rs1/opcode. Documented field layout vector.rs:47.
- Doc contract: vector.rs:47 "Format: [0][vtypei[10:0]][rs1][111][rd][1010111]" — asserted fingerprint 98424e07
- Seed: (none)
- Formal: ∀ rd_a, rd_b, rs1, vtype. (encode(rd_a,rs1,vtype) & ~rd_mask) = (encode(rd_b,rs1,vtype) & ~rd_mask) ∧ rd field equals rd. Symmetric for rs1 and vtypei.
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetvli
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd_a, rd_b, rs1_a, rs1_b, v_a, v_b]
  domain: { rd_a: 0..31, rd_b: 0..31, rs1_a: 0..31, rs1_b: 0..31, v_a: named_vtype, v_b: named_vtype }
  relation:
    op: holds
    lhs: isolation(encode(rd_a,...), encode(rd_b,...))
    rhs: bits outside the varied field are equal
generators:
  rd_a: { gen: int, min: 0, max: 31, type: u32 }
  rd_b: { gen: int, min: 0, max: 31, type: u32 }
  rs1_a: { gen: int, min: 0, max: 31, type: u32 }
  rs1_b: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:47 field layout
```

## encode_vsetvli_neg_arity_fp
- Tier: 3
- Rationale: Negative/error contract. llvm-mc rejects too few operands (`vsetvli a0` / `vsetvli a0, a1`) and FP/vector registers as rd/rs1. rustdoc names rd, rs1 as the first two operands (integer GPRs via get_reg). get_reg returns Err for missing/FP names. Documented error: Result Err(String).
- Doc contract: vector.rs:46 "Encode vsetvli rd, rs1, vtypei" — asserted fingerprint 41263020
- Seed: (none)
- Formal: ∀ ops with len≤2 ∨ rd/rs1 ∈ FP ∪ {v0..v31}. encode_vsetvli(ops) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs
- Status: failing
- Counterexample: encode_vsetvli([Reg("x0"), Reg("x0")]) = Ok(Word(0x7057)); llvm-mc rejects `vsetvli x0, x0` as too few operands
- Bug report: pbt-out/bug_reports/encode_vsetvli_arity_two.md

```property
function: encoder.encode_vsetvli
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, fp]
  domain: { ops: empty_or_one_gpr, fp: fp_or_vreg }
  relation:
    op: throws
    lhs: encode_vsetvli(ops)
    rhs: String
generators:
  ops: { gen: list, elem: { gen: string }, maxLen: 1 }
  fp: { gen: string }
expected_error: String
evidence: vector.rs:49 get_reg; llvm-mc too few operands / invalid operand
```

## encode_vsetvli_neg_extra
- Tier: 3
- Rationale: Negative/error contract. llvm-mc rejects a seventh (or later) operand after a complete named vtype. encode_instruction passes extras through. SUT must Err rather than ignore or retokenize extras (Mem/Imm/Csr skipped or Imm-as-vtypei in parse_vtypei).
- Doc contract: vector.rs:46 "Encode vsetvli rd, rs1, vtypei" — asserted fingerprint 41263020
- Seed: (none)
- Formal: ∀ rd,rs1,sew,lmul,ta,ma, extra. encode_vsetvli([rd,rs1,sew,lmul,ta,ma,extra]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs
- Status: failing
- Counterexample: encode_vsetvli([Reg("x0"), Reg("x0"), Symbol("e8"), Symbol("m1"), Symbol("tu"), Symbol("mu"), Symbol("e8")]) = Ok(Word(0x7057))
- Bug report: pbt-out/bug_reports/encode_vsetvli_extra_operand.md

```property
function: encoder.encode_vsetvli
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, sew, lmul, ta, ma, extra]
  domain: { extra: Operand \ {valid vtype continuation} }
  relation:
    op: throws
    lhs: encode_vsetvli(named ++ [extra])
    rhs: String
generators:
  extra: { gen: string }
expected_error: String
evidence: llvm-mc rejects extra operands; encoder/mod.rs:941 pass-through
```

## encode_vsetvli_diff_llvm_mc_wide_sew
- Tier: 4
- Rationale: Differential covering RVV 1.0 SEW values llvm-mc accepts beyond e8..e64: e128, e256, e512, e1024 (vsew=100/101/110/111). README.md:14 claims V (vector) standard extension. parse_vtypei match lists only e8/e16/e32/e64 — no comment declaring wider SEW invalid (not a domain restriction). Keep the full ISA enumerator set in the generator.
- Doc contract: vector.rs:46 "Encode vsetvli rd, rs1, vtypei" — asserted fingerprint 41263020
- Seed: (none)
- Formal: ∀ rd,rs1 ∈ GPR, sew ∈ {e128,e256,e512,e1024}, lmul ∈ named LMUL, ta,ma. encode_vsetvli(named) = Word(w) ∧ w = llvm-mc(...)
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs
- Status: failing
- Counterexample: encode_vsetvli([Reg("x0"), Reg("x0"), Symbol("e128"), Symbol("m1"), Symbol("tu"), Symbol("mu")]) = Err("unknown vtypei field: e128"); llvm-mc encodes 0x02007057
- Bug report: pbt-out/bug_reports/encode_vsetvli_wide_sew.md

```property
function: encoder.encode_vsetvli
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs1, sew, lmul, ta, ma]
  domain: { sew: {e128,e256,e512,e1024}, lmul: named_lmul, ta: {ta,tu}, ma: {ma,mu} }
  relation:
    op: eq
    lhs: sut_word(named)
    rhs: llvm_mc_word("vsetvli ...")
generators:
  sew: { gen: string }
  lmul: { gen: string }
evidence: vector.rs:46; llvm-mc e128/e256/e512/e1024; RISC-V V 1.0 vsew
```

## encode_vsetvli_neg_fp
- Tier: 3
- Rationale: Sweep — FP/vector rd/rs1 error path was bundled with arity and short-circuited by the arity-2 fail. get_reg (mod.rs:425-431) returns Err for non-integer register names; llvm-mc rejects fa0/v0 as rd/rs1. Negative/error contract.
- Doc contract: vector.rs:46 "Encode vsetvli rd, rs1, vtypei" — asserted fingerprint 41263020
- Seed: (none)
- Formal: ∀ fp ∈ FP ∪ {v0..v31}. encode_vsetvli([Reg(fp), Reg(a1), e32, m1, ta, ma]) = Err(_) ∧ encode_vsetvli([Reg(a0), Reg(fp), e32, m1, ta, ma]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetvli
oracle: negative_error
predicate:
  quantifier: forall
  vars: [fp]
  domain: { fp: fp_or_vreg }
  relation:
    op: throws
    lhs: encode_vsetvli(fp_as_rd_or_rs1)
    rhs: String
generators:
  fp: { gen: string }
expected_error: String
evidence: encoder/mod.rs:425-431 get_reg; llvm-mc invalid operand for ft0/v0
```
