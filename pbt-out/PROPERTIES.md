# Properties: encode_neon_float_elem

## encode_neon_float_elem_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (LLVM 15.0.6) on the GNU-style assembly the assembler claims to accept. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree FP by-element decoder). Sibling encode_neon_elem / encode_neon_float_three_same / encode_fp_arith rejected (same-job gate: integer by-element, vector three-same, and scalar FMUL are different encoding classes).
- Doc contract: neon.rs:1615 "NEON float by-element requires 3 operands" — domain-restriction fingerprint 2f020693
- Seed: encode_neon_elem_pbt.rs:227 encode_neon_elem_diff_llvm_mc
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, idx ∈ {0..imax(T)}, (U,opc,mnem) ∈ {(0,0b1001,fmul),(0,0b0001,fmla),(0,0b0101,fmls),(1,0b1001,fmulx)}. encode_neon_float_elem([Vd.T, Vn.T, Vm.Ts[idx]], U, opc) = llvm-mc("-triple=aarch64 -show-encoding", "mnem Vd.T, Vn.T, Vm.Ts[idx]") where imax(2s)=imax(4s)=3, imax(2d)=1, Ts is s for 2s/4s and d for 2d
- Test file: src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs
- Status: failing
- Counterexample: encode_neon_float_elem([v0.2s, v0.2s, v0.s[0]], U=0, opc=0b1001) = 0x0f009000, llvm-mc("fmul v0.2s, v0.2s, v0.s[0]") = 0x0f809000 (bit 23 clear)
- Bug report: pbt-out/bug_reports/encode_neon_float_elem_size_bit23.md

```property
function: encoder.encode_neon_float_elem
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, idx, u, opcode, mnem]
  domain:
    rd: v0..v31
    t: 2s|4s|2d
  relation:
    op: eq
    lhs: encode_neon_float_elem(ops, u, opcode)
    rhs: llvm_mc(asm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  idx: { gen: int, min: 0, max: 3, type: u32 }
evidence: README.md:12 README.md:231 encoder/mod.rs:456-459 encoder/mod.rs:534-545
```

## encode_neon_float_elem_meta_rd_rn_u
- Tier: 4c
- Rationale: ARM layout isolates Rd at bits[4:0], Rn at bits[9:5], U at bit 29. Stronger differential is the primary property; this metamorphic checks field isolation independently of llvm-mc availability on a given input. Round-trip rejected (no decoder).
- Doc contract: neon.rs:1615 "NEON float by-element requires 3 operands" — domain-restriction fingerprint 2f020693
- Seed: encode_neon_elem_pbt.rs:246 encode_neon_elem_meta_rd_rn_u
- Formal: ∀ rd1,rd2,rn1,rn2,rm ∈ {0..31}, T ∈ {2s,4s,2d}, idx ∈ {0..imax(T)}, opc ∈ {0b0001,0b0101,0b1001}. let w11=encode(rd1,rn1,U=0), w21=encode(rd2,rn1,U=0), w12=encode(rd1,rn2,U=0), wu=encode(rd1,rn1,U=1). (w11 ⊕ w21) ∧ ¬0x1F = 0 ∧ w11[4:0]=rd1 ∧ w21[4:0]=rd2 ∧ (w11 ⊕ w12) ∧ ¬(0x1F≪5) = 0 ∧ w11[9:5]=rn1 ∧ w12[9:5]=rn2 ∧ (w11 ⊕ wu) = 1≪29
- Test file: src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_float_elem
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, rm, t, idx, opcode]
  domain:
    rd1: v0..v31
  relation:
    op: holds
    expr: rd_rn_u_isolated(w11, w21, w12, wu, rd1, rd2, rn1, rn2)
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  opcode: { gen: int, min: 1, max: 9, type: u32 }
evidence: ARM ARM Advanced SIMD vector x indexed element field layout
```

## encode_neon_float_elem_inv_layout
- Tier: 4d
- Rationale: ARM Advanced SIMD vector x indexed element (FP) packing is an exact structural predicate independent of llvm-mc: bit31=0, Q from T, U, bits[28:24]=01111, size=10 for S / 11 for D, L/M/H from index and Rm[4], Rm[4:0], opcode, bit10=0, Rn, Rd. Stronger differential covers end-to-end agreement; this pins each field. Not guessed from the SUT body (the body shifts sz to bit 22 only).
- Doc contract: neon.rs:1615 "NEON float by-element requires 3 operands" — domain-restriction fingerprint 2f020693
- Seed: encode_neon_elem_pbt.rs:282 encode_neon_elem_inv_layout
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, idx ∈ {0..imax(T)}, U ∈ {0,1}, opc ∈ {0b0001,0b0101,0b1001}. let w=encode_neon_float_elem([Vd.T,Vn.T,Vm.Ts[idx]],U,opc). w[31]=0 ∧ w[30]=Q(T) ∧ w[29]=U ∧ w[28:24]=0b01111 ∧ w[23:22]=size(T) ∧ w[21]=L ∧ w[20]=M ∧ w[19:16]=rm[4:0] ∧ w[15:12]=opc ∧ w[11]=H ∧ w[10]=0 ∧ w[9:5]=rn ∧ w[4:0]=rd where size(2s)=size(4s)=0b10, size(2d)=0b11, Q(4s)=Q(2d)=1, Q(2s)=0, and (H,L,M) follow ARM S=H:L / D=H,L=0,M=Rm[4]
- Test file: src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs
- Status: failing
- Counterexample: encode_neon_float_elem([v0.2s, v0.2s, v0.s[0]], U=0, opc=0b0001) has bits[23:22]=00, ARM size for S is 10
- Bug report: pbt-out/bug_reports/encode_neon_float_elem_size_layout.md

```property
function: encoder.encode_neon_float_elem
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, idx, u, opcode]
  domain:
    rd: v0..v31
  relation:
    op: holds
    expr: arm_fp_by_element_layout(w, rd, rn, rm, t, idx, u, opcode)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM Advanced SIMD vector x indexed element FP size 10/11
```

## encode_neon_float_elem_neg_arity
- Tier: 4e
- Rationale: Documented domain restriction: neon.rs:1615 requires 3 operands. Inputs with 0..2 operands must Err. llvm-mc also rejects under-arity FMUL/FMLA/FMLS by-element.
- Doc contract: neon.rs:1615 "NEON float by-element requires 3 operands" — domain-restriction fingerprint 2f020693
- Seed: encode_neon_elem_pbt.rs:322 encode_neon_elem_neg_arity
- Formal: ∀ n ∈ {0,1,2}, ops with |ops|=n (prefix of a valid Vd.T, Vn.T, Vm.Ts[idx] triple). encode_neon_float_elem(ops, U, opc) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_float_elem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, t]
  domain:
    n: 0..2
  relation:
    op: throws
    expr: encode_neon_float_elem(ops_of_len(n), 0, 0b1001)
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
expected_error: String
evidence: neon.rs:1615
```

## encode_neon_float_elem_neg_extra
- Tier: 4e
- Rationale: The documented arity comment states a minimum of 3, not a maximum; gas/llvm-mc reject a fourth operand on FMUL/FMLA/FMLS by-element. README.md:12 claims gas compatibility, so extra operands must Err. The producing len less than 3 check is not Doc evidence that extras are allowed.
- Doc contract: neon.rs:1615 "NEON float by-element requires 3 operands" — domain-restriction fingerprint 2f020693
- Seed: encode_neon_elem_pbt.rs:340 encode_neon_elem_neg_extra
- Formal: ∀ valid (rd,rn,rm,T,idx,U,opc,mnem) and extra ∈ {0..31}. llvm-mc(mnem Vd.T, Vn.T, Vm.Ts[idx], Ve.T) is Err ∧ encode_neon_float_elem([Vd.T,Vn.T,Vm.Ts[idx],Ve.T], U, opc) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs
- Status: failing
- Counterexample: encode_neon_float_elem([v0.2s, v0.2s, v0.s[0], v0.2s], U=0, opc=0b1001) = Ok(Word) while llvm-mc rejects "fmul v0.2s, v0.2s, v0.s[0], v0.2s"
- Bug report: pbt-out/bug_reports/encode_neon_float_elem_extra_operand.md

```property
function: encoder.encode_neon_float_elem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, t, idx, u, opcode, mnem]
  domain:
    extra: v0..v31
  relation:
    op: throws
    expr: encode_neon_float_elem([vd, vn, vm_lane, extra], u, opcode)
generators:
  extra: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: README.md:12 llvm-mc rejects fourth operand
```

## encode_neon_float_elem_neg_mismatch_t
- Tier: 4e
- Rationale: ARM and llvm-mc require dest and source arrangements to match (Vd.T, Vn.T). A mismatched Vn arrangement must Err. The SUT discarding source arrangement is not a contract.
- Doc contract: neon.rs:1615 "NEON float by-element requires 3 operands" — domain-restriction fingerprint 2f020693
- Seed: encode_neon_elem_pbt.rs:372 encode_neon_elem_neg_mismatch_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, T' ≠ T, idx ∈ {0..imax(T)}. llvm-mc("fmul Vd.T, Vn.T', Vm.Ts[idx]") is Err ∧ encode_neon_float_elem([Vd.T, Vn.T', Vm.Ts[idx]], 0, 0b1001) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs
- Status: failing
- Counterexample: encode_neon_float_elem([v0.4s, v0.8b, v0.s[0]], U=0, opc=0b1001) = Ok(Word) while llvm-mc rejects "fmul v0.4s, v0.8b, v0.s[0]"
- Bug report: pbt-out/bug_reports/encode_neon_float_elem_mismatch_t.md

```property
function: encoder.encode_neon_float_elem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, t_wrong, idx]
  domain:
    t: 2s|4s|2d
    t_wrong: other_arrangement
  relation:
    op: throws
    expr: encode_neon_float_elem([Vd.t, Vn.t_wrong, Vm.Ts[idx]], 0, 0b1001)
generators:
  t_wrong: { gen: string }
expected_error: String
evidence: README.md:12 llvm-mc invalid operand for mismatched T
```

## encode_neon_float_elem_neg_gpr_bare_nonv
- Tier: 4e
- Rationale: gas/llvm-mc require Vd.T / Vn.T arranged NEON registers and a lane third operand. GPR dest (x/w), SP, bare V without arrangement, x-prefixed arrangement, scalar s/d dest, and a non-lane third operand must Err.
- Doc contract: neon.rs:1615 "NEON float by-element requires 3 operands" — domain-restriction fingerprint 2f020693
- Seed: encode_neon_elem_pbt.rs:410 encode_neon_elem_neg_gpr_bare_nonv
- Formal: ∀ kind ∈ {x-dest, w-dest, sp-dest, bare-V, x-prefix-arr, s-dest}. llvm-mc(asm(kind)) is Err ∧ encode_neon_float_elem(ops(kind), 0, 0b1001) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs
- Status: failing
- Counterexample: encode_neon_float_elem([RegArrangement(x0,4s), v0.4s, v0.s[0]], U=0, opc=0b1001) = Ok(Word) while llvm-mc rejects "fmul x0.4s, v0.4s, v0.s[0]"
- Bug report: pbt-out/bug_reports/encode_neon_float_elem_x_prefix.md

```property
function: encoder.encode_neon_float_elem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, idx, kind]
  domain:
    kind: 0..6
  relation:
    op: throws
    expr: encode_neon_float_elem(ops_kind, 0, 0b1001)
generators:
  kind: { gen: int, min: 0, max: 6, type: u8 }
expected_error: String
evidence: README.md:12 llvm-mc invalid operand
```

## encode_neon_float_elem_neg_index_oob
- Tier: 4e
- Rationale: llvm-mc rejects vector lane out of range: S lanes [0,3], D lanes [0,1]. ARM index field cannot represent those values without wrapping. Documented bound must be sampled at imax+1.
- Doc contract: neon.rs:1615 "NEON float by-element requires 3 operands" — domain-restriction fingerprint 2f020693
- Seed: encode_neon_elem_pbt.rs:494 encode_neon_elem_neg_index_oob
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, extra ∈ {1..8}. let idx = imax(T)+extra. llvm-mc("fmul Vd.T, Vn.T, Vm.Ts[idx]") is Err ∧ encode_neon_float_elem([Vd.T,Vn.T,Vm.Ts[idx]], 0, 0b1001) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs
- Status: failing
- Counterexample: encode_neon_float_elem([v0.2s, v0.2s, v0.s[4]], U=0, opc=0b1001) = Ok(Word) while llvm-mc rejects "fmul v0.2s, v0.2s, v0.s[4]" (lane range [0, 3])
- Bug report: pbt-out/bug_reports/encode_neon_float_elem_index_oob.md

```property
function: encoder.encode_neon_float_elem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, extra]
  domain:
    extra: 1..8
  relation:
    op: throws
    expr: encode_neon_float_elem([Vd.t, Vn.t, Vm.Ts[imax_plus_extra]], 0, 0b1001)
generators:
  extra: { gen: int, min: 1, max: 8, type: u32 }
expected_error: String
evidence: llvm-mc vector lane must be an integer in range
```

## encode_neon_float_elem_neg_unsupported_t
- Tier: 4e
- Rationale: neon.rs:1624 declares arrangements other than 2s/4s/2d unsupported. Sweep covers that documented error path (coverage_gaps had no Rust profraw).
- Doc contract: neon.rs:1624 "float by-element: unsupported:" — domain-restriction fingerprint d3f34d13
- Seed: encode_neon_elem_pbt.rs encode_neon_elem_neg_unsupported_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b,4h,8h,1d,4b,ε}, idx ∈ {0..3}. encode_neon_float_elem([Vd.T, Vn.T, Vm.s[idx]], 0, 0b1001) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_float_elem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, idx, t]
  domain:
    t: unsupported_arrangement
  relation:
    op: throws
    expr: encode_neon_float_elem([Vd.t, Vn.t, Vm.s[idx]], 0, 0b1001)
generators:
  t: { gen: string }
expected_error: String
evidence: neon.rs:1624
```

## encode_neon_float_elem_meta_alt_spellings
- Tier: 4c
- Rationale: parse_reg_num lowercases names, so uppercase V must encode identically to lowercase v. Not vs llvm-mc (that is the already-filed size-bit bug).
- Doc contract: neon.rs:1615 "NEON float by-element requires 3 operands" — domain-restriction fingerprint 2f020693
- Seed: encode_neon_elem_pbt.rs encode_neon_elem_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, idx ∈ {0..imax(T)}, (U,opc) ARM table. encode(V-prefix) = encode(v-prefix)
- Test file: src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_float_elem
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, idx, u, opcode]
  domain:
    rd: v0..v31
  relation:
    op: eq
    lhs: encode_neon_float_elem(upper_V_ops, u, opcode)
    rhs: encode_neon_float_elem(lower_v_ops, u, opcode)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: parse_reg_num lowercases register names
```

## encode_neon_float_elem_neg_lane_elem_mismatch
- Tier: 4e
- Rationale: llvm-mc requires the lane element size to match T (s for 2s/4s, d for 2d). The SUT discards elem_size. Documented gas compatibility requires Err.
- Doc contract: neon.rs:1615 "NEON float by-element requires 3 operands" — domain-restriction fingerprint 2f020693
- Seed: encode_neon_elem_pbt.rs encode_neon_elem_neg_lane_elem_mismatch
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, wrong ≠ Ts(T), idx ∈ {0..imax(T)}. llvm-mc("fmul Vd.T, Vn.T, Vm.wrong[idx]") is Err ∧ encode_neon_float_elem([Vd.T, Vn.T, Vm.wrong[idx]], 0, 0b1001) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_float_elem_pbt.rs
- Status: failing
- Counterexample: encode_neon_float_elem([v0.2d, v0.2d, v0.b[0]], U=0, opc=0b1001) = Ok(Word) while llvm-mc rejects "fmul v0.2d, v0.2d, v0.b[0]"
- Bug report: pbt-out/bug_reports/encode_neon_float_elem_lane_elem.md

```property
function: encoder.encode_neon_float_elem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, idx, wrong]
  domain:
    wrong: b|h|s|d minus matching elem
  relation:
    op: throws
    expr: encode_neon_float_elem([Vd.t, Vn.t, Vm.wrong[idx]], 0, 0b1001)
generators:
  wrong: { gen: string }
expected_error: String
evidence: README.md:12 llvm-mc invalid operand for mismatched lane size
```
