# Properties: encode_mov

## encode_mov_diff_gpr_reg
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc/gas on integer register MOV (ORR XZR / ADD SP). State machine rejected (pure function). Round-trip rejected (no MOV decoder). encode_movz/orr siblings rejected (same-job gate).
- Doc contract: data_processing.rs:123 "mov Xd, Xm -> ORR Xd, XZR, Xm" — asserted fingerprint 901d3010; data_processing.rs:129 "Check for MOV to/from SP: uses ADD Xd, Xn, #0" — asserted fingerprint f4ba2f4a; README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438
- Seed: data_processing.rs encode_movk_pbt llvm-mc GPR mapping
- Formal: ∀ rd,rm ∈ 0..31, is_64 ∈ {0,1}, rd_sp,rm_sp ∈ {0,1}. encode_mov([Reg(gpr(is_64,rd,rd_sp)), Reg(gpr(is_64,rm,rm_sp))]) = Word(llvm-mc("mov Rd, Rm"))
- Test file: src/backend/arm/assembler/encoder/encode_mov_pbt.rs
- Status: failing
- Counterexample: encode_mov([Reg("wsp"), Reg("w0")]) → Word(0x2a0003ff) vs llvm-mc Word(0x1100031f)
- Bug report: pbt-out/bug_reports/encode_mov_wsp_as_wzr.md

```property
function: encode_mov
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rm, is_64, rd_sp, rm_sp]
  domain: { rd: 0..31, rm: 0..31, is_64: bool, rd_sp: bool, rm_sp: bool }
  relation:
    op: eq
    lhs: encode_mov([Reg(gpr(is_64,rd,rd_sp)), Reg(gpr(is_64,rm,rm_sp))])
    rhs: Word(llvm_mc("mov " + gpr(is_64,rd,rd_sp) + ", " + gpr(is_64,rm,rm_sp)))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  rd_sp: { gen: bool }
  rm_sp: { gen: bool }
evidence: README.md:12 gas contract; data_processing.rs:123,129
```

## encode_mov_diff_imm
- Tier: 5
- Rationale: Differential vs llvm-mc on width-appropriate MOV immediates. Alias encodings (MOVZ vs MOVN vs ORR-bitmask) of the same materialized value are allowed by README.md:287 search order. Words vs llvm-mc-reject uses ARM move-wide reconstruct (README expansion).
- Doc contract: data_processing.rs:85 "mov Xd, #imm -> movz or movn" — asserted fingerprint db7801db; README.md:287 "Wide immediates: `mov Xd, #large` first tries single-instruction encodings" — asserted fingerprint 36936730
- Seed: encode_movz_pbt / encode_movk_pbt immediate mapping
- Formal: ∀ rd ∈ 0..30, (is_64, imm) ∈ width-appropriate domain. llvm-mc("mov Rd, #imm")=w ∧ encode_mov=Word(s) ⇒ s=w ∨ materialized(s)=imm. llvm-mc rejects ∧ encode_mov=Words(ws) ⇒ reconstruct(ws)=imm
- Test file: src/backend/arm/assembler/encoder/encode_mov_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, is_64, imm]
  domain: { rd: 0..30, (is_64, imm): width-appropriate mov immediates }
  relation:
    op: eq
    lhs: materialized(encode_mov([Reg(gpr(is_64,rd,false)), Imm(imm)]))
    rhs: imm
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: README.md:12; README.md:287; data_processing.rs:85
```

## encode_mov_diff_neon
- Tier: 5
- Rationale: Differential vs llvm-mc on NEON MOV aliases documented at data_processing.rs:11/24/46/64 (vector ORR 8b/16b, INS from GPR, UMOV to GPR .s/.d, INS element).
- Doc contract: data_processing.rs:11 "NEON register-to-register move: mov v1.16b, v0.16b -> ORR v1.16b, v0.16b, v0.16b" — asserted fingerprint 17a6a52e
- Seed: encode_neon_ins_pbt / encode_neon_umov_pbt
- Formal: ∀ valid NEON MOV form F ∈ {8b/16b vector, INS-GPR, UMOV S/D, INS-elem}. encode_mov(ops(F)) = Word(llvm-mc(asm(F)))
- Test file: src/backend/arm/assembler/encoder/encode_mov_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov
oracle: differential
predicate:
  quantifier: forall
  vars: [form]
  domain: { form: neon_mov_forms }
  relation:
    op: eq
    lhs: encode_mov(ops(form))
    rhs: Word(llvm_mc(asm(form)))
generators:
  form: { gen: oneof }
evidence: data_processing.rs:11,24,46,64; README.md:293
```

## encode_mov_metamorphic_sf
- Tier: 4
- Rationale: Algebraic metamorphic: same rd/rm numbers, X vs W (neither SP), register MOV words differ only in sf bit 31.
- Doc contract: README.md size-inference paragraph — asserted (sf from register prefix)
- Seed: encode_movk_pbt encode_movk_metamorphic_sf
- Formal: ∀ rd,rm ∈ 0..30. encode_mov(Xrd,Xrm) XOR encode_mov(Wrd,Wrm) = 1<<31
- Test file: src/backend/arm/assembler/encoder/encode_mov_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rm]
  domain: { rd: 0..30, rm: 0..30 }
  relation:
    op: eq
    lhs: encode_mov([Reg("x"+rd), Reg("x"+rm)]) XOR encode_mov([Reg("w"+rd), Reg("w"+rm)])
    rhs: 1 << 31
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
evidence: README.md size-inference sf bit; ARM ARM ORR sf
```

## encode_mov_invariant_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM / inline comments: non-SP register MOV is ORR Rd,XZR,Rm; SP/WSP form is ADD Rd,Rn,#0.
- Doc contract: data_processing.rs:123 "mov Xd, Xm -> ORR Xd, XZR, Xm" — asserted fingerprint 901d3010; data_processing.rs:129 "Check for MOV to/from SP: uses ADD Xd, Xn, #0" — asserted fingerprint f4ba2f4a
- Seed: encode_movk_pbt encode_movk_invariant_arm_fields
- Formal: ∀ rd,rm ∈ 0..31, is_64. WSP/SP ⇒ ADD layout (op=0,S=0,opc=10001,imm12=0). else ⇒ ORR layout (opc=01, 01010, N=0, Rn=31)
- Test file: src/backend/arm/assembler/encoder/encode_mov_pbt.rs
- Status: failing
- Counterexample: encode_mov([Reg("wsp"), Reg("w0")]) does not have ADD S=0 (encodes ORR)
- Bug report: pbt-out/bug_reports/encode_mov_wsp_add_layout.md

```property
function: encode_mov
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rm, is_64, rd_sp, rm_sp]
  domain: { rd: 0..31, rm: 0..31 }
  relation:
    op: holds
    expr: arm_orr_or_add_layout(encode_mov(ops))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  rd_sp: { gen: bool }
  rm_sp: { gen: bool }
evidence: data_processing.rs:123,129; ARM ARM C6 MOV (register) / ADD (immediate)
```

## encode_mov_neg_arity
- Tier: 3
- Rationale: Negative/error: body returns Err when operands.len() < 2.
- Doc contract: data_processing.rs:7-8 arity lower bound — asserted
- Seed: encode_movk_pbt encode_movk_neg_too_few
- Formal: ∀ ops. |ops| ∈ {0,1} ⇒ encode_mov(ops) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_mov_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: lists of length 0 or 1 }
  relation:
    op: throws
    expr: encode_mov(ops)
    error: String
generators:
  ops: { gen: list, maxLen: 1 }
expected_error: String
evidence: data_processing.rs:7-8; GNU as rejects 0/1-operand mov
```

## encode_mov_neg_extra
- Tier: 3
- Rationale: Negative/error under README.md:12 gas contract. GNU as rejects extra operands.
- Doc contract: README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438
- Seed: encode_movk_pbt encode_movk_neg_extra_operand
- Formal: ∀ rd,rm ∈ 0..30, extra. encode_mov([Reg(Xrd), Reg(Xrm), extra]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_mov_pbt.rs
- Status: failing
- Counterexample: encode_mov([Reg("x0"), Reg("x0"), Reg("x0")]) → Ok(Word(0xaa0003e0))
- Bug report: pbt-out/bug_reports/encode_mov_extra_operand.md

```property
function: encode_mov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rm, extra]
  domain: { rd: 0..30, rm: 0..30 }
  relation:
    op: throws
    expr: encode_mov([Reg("x"+rd), Reg("x"+rm), extra])
    error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: README.md:12; GNU as extra-operand rejection
```

## encode_mov_neg_mixed
- Tier: 3
- Rationale: Negative/error: gas rejects mixed X/W MOV.
- Doc contract: README.md:12 gas contract — asserted fingerprint f00ab438
- Seed: encode_movk_pbt mixed-width negatives
- Formal: ∀ rd,rm ∈ 0..30. encode_mov([Reg("x"+rd), Reg("w"+rm)]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_mov_pbt.rs
- Status: failing
- Counterexample: encode_mov([Reg("x0"), Reg("w0")]) → Ok(Word(0xaa0003e0))
- Bug report: pbt-out/bug_reports/encode_mov_mixed_width.md

```property
function: encode_mov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rm]
  domain: { rd: 0..30, rm: 0..30 }
  relation:
    op: throws
    expr: encode_mov([Reg("x"+rd), Reg("w"+rm)])
    error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: README.md:12; GNU as operand mismatch
```

## encode_mov_neg_fp
- Tier: 3
- Rationale: Negative/error: gas rejects `mov d0, d1` (use fmov).
- Doc contract: README.md:12 gas contract — asserted fingerprint f00ab438
- Seed: encode_movk_pbt encode_movk_neg_fp
- Formal: ∀ n ∈ 0..31. encode_mov([Reg("d"+n), Reg("d"+(n+1)%32)]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_mov_pbt.rs
- Status: failing
- Counterexample: encode_mov([Reg("d0"), Reg("d1")]) → Ok(Word(0x2a0103e0))
- Bug report: pbt-out/bug_reports/encode_mov_fp_as_gpr.md

```property
function: encode_mov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [fp_n]
  domain: { fp_n: 0..31 }
  relation:
    op: throws
    expr: encode_mov([Reg("d"+fp_n), Reg("d"+(fp_n+1)%32)])
    error: String
generators:
  fp_n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: README.md:12; GNU as FP-scalar rejection
```

## encode_mov_neg_sp_imm
- Tier: 3
- Rationale: Negative/error: gas/llvm-mc reject `mov sp, #imm` (MOVZ Rd cannot be SP).
- Doc contract: README.md:12 gas contract — asserted fingerprint f00ab438
- Seed: encode_movk_pbt encode_movk_neg_sp
- Formal: ∀ imm. encode_mov([Reg("sp"), Imm(imm)]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_mov_pbt.rs
- Status: failing
- Counterexample: encode_mov([Reg("sp"), Imm(0)]) → Ok(Word(0xd28003ff))
- Bug report: pbt-out/bug_reports/encode_mov_sp_imm.md

```property
function: encode_mov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: i64 }
  relation:
    op: throws
    expr: encode_mov([Reg("sp"), Imm(imm)])
    error: String
generators:
  imm: { gen: int, min: -65536, max: 65535, type: i64 }
expected_error: String
evidence: README.md:12; llvm-mc/gas reject mov sp, #imm
```

## encode_mov_neg_lane_oob
- Tier: 3
- Rationale: Negative/error: llvm-mc requires lane in [0,15] for .b.
- Doc contract: README.md:12 gas contract — asserted fingerprint f00ab438
- Seed: encode_neon_ins_pbt index range
- Formal: ∀ vd ∈ 0..31, rm ∈ 0..30, idx ∈ 16..31. encode_mov([RegLane(v_vd, b, idx), Reg("w"+rm)]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_mov_pbt.rs
- Status: failing
- Counterexample: encode_mov([RegLane(v0, b, 16), Reg("w0")]) → Ok(Word(0x4e010c00))
- Bug report: pbt-out/bug_reports/encode_mov_lane_oob.md

```property
function: encode_mov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, rm, idx]
  domain: { vd: 0..31, rm: 0..30, idx: 16..31 }
  relation:
    op: throws
    expr: encode_mov([RegLane(vd,"b",idx), Reg("w"+rm)])
    error: String
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  idx: { gen: int, min: 16, max: 31, type: u32 }
expected_error: String
evidence: README.md:12; llvm-mc lane range
```

## encode_mov_neg_arr_mismatch
- Tier: 3
- Rationale: Negative/error: llvm-mc rejects `mov v0.16b, v1.8b`.
- Doc contract: data_processing.rs:11 8b/16b ORR alias — asserted fingerprint 17a6a52e
- Seed: encode_neon_ins_pbt arrangement match
- Formal: ∀ vd,vn ∈ 0..31. encode_mov([RegArrangement(vd,16b), RegArrangement(vn,8b)]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_mov_pbt.rs
- Status: failing
- Counterexample: encode_mov([RegArrangement(v0,16b), RegArrangement(v0,8b)]) → Ok(Word(0x4ea01c00))
- Bug report: pbt-out/bug_reports/encode_mov_arr_mismatch.md

```property
function: encode_mov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vn]
  domain: { vd: 0..31, vn: 0..31 }
  relation:
    op: throws
    expr: encode_mov([RegArrangement(vd,"16b"), RegArrangement(vn,"8b")])
    error: String
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: README.md:12; llvm-mc arrangement mismatch
```

## encode_mov_neg_vec_4s
- Tier: 3
- Rationale: Negative/error: GNU as rejects vector MOV except 8b/16b. SUT encodes 4s with Q=0, which is also wrong vs llvm-mc Q=1.
- Doc contract: README.md:12 gas contract — asserted fingerprint f00ab438; data_processing.rs:11 16b example — asserted fingerprint 17a6a52e
- Seed: encode_neon vector arrangement
- Formal: ∀ vd,vn ∈ 0..31. encode_mov([RegArrangement(vd,4s), RegArrangement(vn,4s)]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_mov_pbt.rs
- Status: failing
- Counterexample: encode_mov([RegArrangement(v0,4s), RegArrangement(v0,4s)]) → Ok(Word(0x0ea01c00))
- Bug report: pbt-out/bug_reports/encode_mov_vec_4s.md

```property
function: encode_mov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vn]
  domain: { vd: 0..31, vn: 0..31 }
  relation:
    op: throws
    expr: encode_mov([RegArrangement(vd,"4s"), RegArrangement(vn,"4s")])
    error: String
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: README.md:12; GNU as 8b/16b-only vector MOV
```

## encode_mov_neg_w_large_imm
- Tier: 3
- Rationale: Negative/error: gas/llvm-mc reject a 64-bit literal on a W dest. SUT truncates via 32-bit bitmask.
- Doc contract: README.md:12 gas contract — asserted fingerprint f00ab438
- Seed: encode_mov diff_imm W domain
- Formal: ∀ rd ∈ 0..30, imm with high 32 bits nonzero and not a 32-bit sign-extend. encode_mov([Reg("w"+rd), Imm(imm)]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_mov_pbt.rs
- Status: failing
- Counterexample: encode_mov([Reg("w0"), Imm(0x0101010101010101)]) → Ok(Word(0x3200c3e0))
- Bug report: pbt-out/bug_reports/encode_mov_w_large_imm.md

```property
function: encode_mov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: 0..30, imm: 64-bit-only literals }
  relation:
    op: throws
    expr: encode_mov([Reg("w"+rd), Imm(imm)])
    error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  imm: { gen: int, type: i64 }
expected_error: String
evidence: README.md:12; GNU as "immediate cannot be moved by a single instruction"
```

## encode_mov_diff_alt_spellings
- Tier: 5
- Rationale: Sweep: lr / uppercase Xn aliases must match llvm-mc (parse_reg_num).
- Doc contract: README.md:275 register parsing includes lr — asserted
- Seed: encode_adrp_pbt encode_adrp_diff_alt_spellings
- Formal: ∀ n,m ∈ 0..30. encode_mov([Reg(alias(n)), Reg(alias(m))]) = Word(llvm-mc("mov alias(n), alias(m)"))
- Test file: src/backend/arm/assembler/encoder/encode_mov_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov
oracle: differential
predicate:
  quantifier: forall
  vars: [n, m, use_lr, upper]
  domain: { n: 0..30, m: 0..30 }
  relation:
    op: eq
    lhs: encode_mov([Reg(alias(n)), Reg(alias(m))])
    rhs: Word(llvm_mc("mov " + alias(n) + ", " + alias(m)))
generators:
  n: { gen: int, min: 0, max: 30, type: u32 }
  m: { gen: int, min: 0, max: 30, type: u32 }
evidence: README.md register parsing; parse_reg_num lr / case-fold
```
