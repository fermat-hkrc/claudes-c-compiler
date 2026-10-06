# Properties: encode_vsetvl

## encode_vsetvl_diff_llvm_mc
- Tier: 4
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent RISC-V assembler, same job: assemble `vsetvl` to a 32-bit word). State machine rejected — pure function, no lifecycle. Algebraic round-trip rejected — no in-tree vsetvl decoder. encode_vsetvli / encode_vsetivli rejected as siblings — same-job gate fails (vsetvli bit31=0 and vtypei immediate; vsetivli bits[31:30]=11 and uimm AVL). Wrapper encode_instruction passes operands through (mod.rs:949).
- Doc contract: vector.rs:68-69 "Encode vsetvl rd, rs1, rs2" / "Format: [1000000][rs2][rs1][111][rd][1010111]" — asserted fingerprint 1f499689
- Seed: encode_vsetvli_pbt.rs named differential (generalized to three GPRs, no vtypei)
- Formal: ∀ rd, rs1, rs2 ∈ {x0..x31} ∪ ABI ∪ {fp}. encode_vsetvl([Reg(rd), Reg(rs1), Reg(rs2)]) = Word(w) ∧ w = llvm-mc("vsetvl rd, rs1, rs2", triple=riscv64, mattr=+v)
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetvl
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs1, rs2]
  domain: { rd: gpr_name, rs1: gpr_name, rs2: gpr_name }
  relation:
    op: eq
    lhs: sut_word([Reg(rd), Reg(rs1), Reg(rs2)])
    rhs: llvm_mc_word("vsetvl {rd}, {rs1}, {rs2}")
generators:
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
evidence: vector.rs:68-69; encoder/mod.rs:949; assembler/README.md:14; llvm-mc -triple=riscv64 -mattr=+v
```

## encode_vsetvl_format_fields
- Tier: 3
- Rationale: Algebraic invariant from the rustdoc format line and RISC-V V 1.0: opcode=1010111, funct3=111, bits[31:25]=1000000, rd in [11:7], rs1 in [19:15], rs2 in [24:20]. Weaker than differential; kept as an llvm-mc-independent structural check. Round-trip rejected (no decoder). Documented bounds 0 and 31 for each 5-bit register field are sampled exactly via gpr_n 0..=31.
- Doc contract: vector.rs:68-69 "Format: [1000000][rs2][rs1][111][rd][1010111]" — asserted fingerprint 05cee612
- Seed: encode_vsetvli_pbt.rs format_fields
- Formal: ∀ rd, rs1, rs2 ∈ 0..31. let w = encode_vsetvl([Reg(x{rd}), Reg(x{rs1}), Reg(x{rs2})]). w[6:0]=1010111 ∧ w[14:12]=111 ∧ w[31:25]=1000000 ∧ w[11:7]=rd ∧ w[19:15]=rs1 ∧ w[24:20]=rs2
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetvl
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, rs2]
  domain: { rd: 0..=31, rs1: 0..=31, rs2: 0..=31 }
  relation:
    op: holds
    expr: unpack_vsetvl(sut_word([Reg(xn(rd)), Reg(xn(rs1)), Reg(xn(rs2))])) == (OP_V, rd, 0b111, rs1, rs2, 0b1000000)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:68-69 RISC-V V 1.0 vsetvl layout
```

## encode_vsetvl_abi_xn_alias
- Tier: 3
- Rationale: Algebraic metamorphic: ABI names (zero/ra/sp/…/fp) must encode the same word as xN. RISC-V register aliases are the same 5-bit encoding. Differential already covers llvm-mc agreement; this is an llvm-mc-independent alias identity. Stronger round-trip rejected (no decoder).
- Doc contract: vector.rs:68-69 "Encode vsetvl rd, rs1, rs2" — asserted fingerprint 5f5ce8ca
- Seed: encode_vsetvli_pbt.rs abi_xn_alias
- Formal: ∀ n, m, k ∈ 0..31. encode_vsetvl([Reg(ABI[n]), Reg(ABI[m]), Reg(ABI[k])]) = encode_vsetvl([Reg(x{n}), Reg(x{m}), Reg(x{k})]) ∧ (n=8 ⇒ encode_vsetvl([Reg("fp"), …]) equals the x8 form)
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetvl
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, k]
  domain: { n: 0..=31, m: 0..=31, k: 0..=31 }
  relation:
    op: eq
    lhs: sut_word([Reg(abi(n)), Reg(abi(m)), Reg(abi(k))])
    rhs: sut_word([Reg(xn(n)), Reg(xn(m)), Reg(xn(k))])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  k: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:228-272 ABI names and fp|s0 map to the same 5-bit numbers as xN
```

## encode_vsetvl_field_isolation
- Tier: 3
- Rationale: Algebraic metamorphic: changing only rd (resp. rs1, rs2) must leave all non-rd (resp. non-rs1, non-rs2) bits unchanged. Independent of llvm-mc. Documented 5-bit field bounds 0 and 31 sampled via gpr_n.
- Doc contract: vector.rs:68-69 "Format: [1000000][rs2][rs1][111][rd][1010111]" — asserted fingerprint 05cee612
- Seed: encode_vsetvli_pbt.rs field_isolation
- Formal: ∀ rd_a, rd_b, rs1_a, rs1_b, rs2_a, rs2_b ∈ 0..31. let wa = encode(rd_a,rs1_a,rs2_a). changing rd leaves bits outside [11:7] equal; changing rs1 leaves bits outside [19:15] equal; changing rs2 leaves bits outside [24:20] equal
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetvl
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd_a, rd_b, rs1_a, rs1_b, rs2_a, rs2_b]
  domain: { rd_a: 0..=31, rd_b: 0..=31, rs1_a: 0..=31, rs1_b: 0..=31, rs2_a: 0..=31, rs2_b: 0..=31 }
  relation:
    op: holds
    expr: (wa & !rd_mask) == (wb & !rd_mask) && (wa & !rs1_mask) == (wc & !rs1_mask) && (wa & !rs2_mask) == (wd & !rs2_mask)
generators:
  rd_a: { gen: int, min: 0, max: 31, type: u32 }
  rd_b: { gen: int, min: 0, max: 31, type: u32 }
  rs1_a: { gen: int, min: 0, max: 31, type: u32 }
  rs1_b: { gen: int, min: 0, max: 31, type: u32 }
  rs2_a: { gen: int, min: 0, max: 31, type: u32 }
  rs2_b: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:68-69 disjoint field layout
```

## encode_vsetvl_neg_arity_fp
- Tier: 2
- Rationale: Negative/error contract: rustdoc names three GPR operands; llvm-mc rejects too few operands and FP/vector registers as invalid operand. Wrapper passes operands through. Stronger differential does not apply on the error path (llvm-mc has no encoding). Empty / 1 / 2 operand lists and fN / vN / fa0 / v0 as any of rd, rs1, rs2 must Err.
- Doc contract: vector.rs:68-69 "Encode vsetvl rd, rs1, rs2" — asserted fingerprint 5f5ce8ca
- Seed: encode_vsetvli_pbt.rs neg_arity_fp
- Formal: ∀ ops with |ops|<3. encode_vsetvl(ops) = Err. ∀ fp ∈ FP∪VREG, pos ∈ {rd,rs1,rs2}. encode_vsetvl with fp at pos = Err
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetvl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, fp]
  domain: { ops: arity 0..=2 GPR lists, fp: fN|vN|ABI-FP }
  relation:
    op: throws
    expr: encode_vsetvl(ops)
    error: String
expected_error: String
generators:
  ops: { gen: list }
  fp: { gen: string }
evidence: vector.rs:68; llvm-mc "too few operands" / "invalid operand"; encoder/mod.rs:429 get_reg rejects non-integer regs
```

## encode_vsetvl_neg_extra
- Tier: 2
- Rationale: Negative/error contract: rustdoc names exactly three operands; llvm-mc rejects a fourth (`invalid operand for instruction`). Wrapper encode_instruction passes operands through. encode_vsetvl currently reads only indices 0,1,2 via get_reg and ignores extras — that is the candidate defect, not a reason to drop the property.
- Doc contract: vector.rs:68-69 "Encode vsetvl rd, rs1, rs2" — asserted fingerprint 5f5ce8ca
- Seed: encode_vsetvli_pbt.rs neg_extra
- Formal: ∀ rd, rs1, rs2 ∈ GPR names, extra ∈ Operand. encode_vsetvl([Reg(rd), Reg(rs1), Reg(rs2), extra]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvl_pbt.rs
- Status: failing
- Counterexample: encode_vsetvl([Reg("x0"), Reg("x0"), Reg("x0"), Imm(0)]) → Ok(Word(0x80007057))
- Bug report: pbt-out/bug_reports/encode_vsetvl_extra_operand.md

```property
function: encoder.encode_vsetvl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, rs2, extra]
  domain: { rd: gpr_name, rs1: gpr_name, rs2: gpr_name, extra: Operand }
  relation:
    op: throws
    expr: encode_vsetvl([Reg(rd), Reg(rs1), Reg(rs2), extra])
    error: String
expected_error: String
generators:
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
  extra: { gen: string }
evidence: vector.rs:68 three-operand form; llvm-mc rejects extra operand; encoder/mod.rs:949 pass-through
```

## encode_vsetvl_neg_nonreg
- Tier: 2
- Rationale: Negative/error contract: each of rd, rs1, rs2 must be an integer register. llvm-mc rejects immediates, symbols, and memory as operands. Imm(0..=31) is excluded from this generator — get_reg (mod.rs:434-435) documents GCC bare GPR numbers as accepted, so that input is out of the invalid domain (quirk, not a restriction of encode_vsetvl's rustdoc). Other non-Reg kinds (Imm outside 0..=31, Symbol, Label, Mem, Csr, FenceArg, RoundingMode, SymbolOffset) must Err at every position.
- Doc contract: vector.rs:68-69 "Encode vsetvl rd, rs1, rs2" — asserted fingerprint 5f5ce8ca
- Seed: encode_vsetvli_pbt.rs extra_operand kinds reused as non-register operands
- Formal: ∀ pos ∈ {0,1,2}, bad ∈ non-Reg Operand kinds excluding Imm(0..=31). encode_vsetvl with bad at pos (other positions valid GPR) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_vsetvl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vsetvl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [pos, bad]
  domain: { pos: 0..=2, bad: non-Reg Operand excluding Imm(0..=31) }
  relation:
    op: throws
    expr: encode_vsetvl(ops_with_bad_at(pos, bad))
    error: String
expected_error: String
generators:
  pos: { gen: int, min: 0, max: 2, type: usize }
  bad: { gen: string }
evidence: vector.rs:68 GPR operands; llvm-mc invalid operand; encoder/mod.rs:429 get_reg
```
