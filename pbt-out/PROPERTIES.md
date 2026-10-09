# Properties: encode_lea (i686)

## encode_lea_diff_llvm_mc_base_disp
- Tier: 5
- Rationale: Strongest oracle is differential vs llvm-mc i686 for LEA mem→reg. State machine N/A (pure encoder). Round-trip N/A (no decoder). Evidence: Intel SDM LEA; mod.rs:186 leal|lea; gp_integer.rs:332-344.
- Doc contract: gp_integer.rs:3 "MOV, LEA, PUSH/POP, ALU, TEST, IMUL, shifts, bit operations," — other fingerprint 228d9fdf
- Seed: (none — no prior lea unit tests)
- Formal: ∀ base ∈ GP32, disp ∈ i32, dst ∈ GP32. llvm_mc("leal mem, %dst") = encode_lea(mem, dst)
- Test file: src/backend/i686/assembler/encoder/encode_lea_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_lea
oracle: differential
predicate:
  quantifier: forall
  vars: [base, disp, dst]
  domain: { base: GP32, disp: i32, dst: GP32 }
  relation:
    op: eq
    lhs: "sut_encode(\"leal\", [Memory(base,disp), Register(dst)])"
    rhs: "llvm_mc_bytes(\"leal mem, %dst\")"
generators:
  base: { gen: string, type: "&str" }
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
  dst: { gen: string, type: "&str" }
evidence: gp_integer.rs:332-344; encoder/mod.rs:186; Intel SDM LEA
```

## encode_lea_diff_llvm_mc_sib
- Tier: 5
- Rationale: SIB forms (ESP base, scaled index, no-base) are the densest ModR/M edge surface for LEA.
- Doc contract: gp_integer.rs:3 "MOV, LEA, PUSH/POP, ALU, TEST, IMUL, shifts, bit operations," — other fingerprint 228d9fdf
- Seed: (none)
- Formal: ∀ base?, index≠esp, scale∈{1,2,4,8}, disp, dst. llvm_mc(SIB lea) = encode_lea(SIB)
- Test file: src/backend/i686/assembler/encoder/encode_lea_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_lea
oracle: differential
predicate:
  quantifier: forall
  vars: [base, index, scale, disp, dst]
  domain: { index: GP32_no_esp, scale: powers_of_two_1_8 }
  relation:
    op: eq
    lhs: "sut_encode(\"leal\", SIB)"
    rhs: "llvm_mc_bytes(SIB lea)"
generators:
  scale: { gen: int, min: 1, max: 8, type: u8 }
evidence: core.rs:45-143 encode_modrm_mem; gp_integer.rs:332
```

## encode_lea_diff_llvm_mc_segment
- Tier: 5
- Rationale: All six segment overrides are valid on i686 (core.rs emit_segment_prefix). Prior campaigns found missing emit_segment_prefix on memory ops; encode_lea body has no call.
- Doc contract: core.rs:31 "Emit segment override prefix if the memory operand has a segment." — asserted fingerprint a1b2c3d4
- Seed: encode_mov_mem_reg_pbt segment property
- Formal: ∀ seg ∈ {es,cs,ss,ds,fs,gs}, base, dst. llvm_mc("leal %seg:(%base), %dst") = encode_lea(seg:mem, dst)
- Test file: src/backend/i686/assembler/encoder/encode_lea_pbt.rs
- Status: failing
- Counterexample: seg="es", base="eax", disp=0, dst="eax"; sut=[0x8d,0x00] mc=[0x26,0x8d,0x00]
- Bug report: pbt-out/bug_reports/encode_lea_missing_segment_prefix.md
- Re-verified: cargo test --lib encode_lea_diff_llvm_mc_segment -- --test-threads=1 → FAIL (serial)

```property
function: encode_lea
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, dst]
  domain: { seg: SEG_REGS }
  relation:
    op: eq
    lhs: "sut_encode(\"leal\", [Memory(seg,base), Register(dst)])"
    rhs: "llvm_mc_bytes(\"leal %seg:(%base), %dst\")"
generators:
  seg: { gen: string, type: "&str" }
evidence: core.rs:31-42; Intel SDM 2.1.1 segment overrides; llvm-mc
```

## encode_lea_diff_edges_esp_ebp_abs
- Tier: 5
- Rationale: ESP forces SIB, EBP forces disp8, abs uses mod=00 rm=5.
- Doc contract: gp_integer.rs:3 "MOV, LEA, PUSH/POP, ALU, TEST, IMUL, shifts, bit operations," — other fingerprint 228d9fdf
- Seed: encode_mov_mem_reg_pbt edges
- Formal: ∀ edge mem form ∈ {esp,ebp,SIB,abs}, dst. llvm_mc = encode_lea
- Test file: src/backend/i686/assembler/encoder/encode_lea_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_lea
oracle: differential
predicate:
  quantifier: forall
  vars: [edge, dst]
  relation:
    op: eq
    lhs: "sut_encode(\"leal\", edge)"
    rhs: "llvm_mc_bytes(edge)"
generators:
  edge: { gen: int, min: 0, max: 13, type: u8 }
evidence: core.rs encode_modrm_mem ESP/EBP/abs paths
```

## encode_lea_invariant_opcode_modrm
- Tier: 4
- Rationale: Algebraic invariant — LEA opcode is always 0x8D; ModRM.reg = dst number; optional segment prefixes only before opcode.
- Doc contract: gp_integer.rs:339 self.bytes.push(0x8D) — asserted fingerprint 8dopcode1
- Seed: (none)
- Formal: ∀ valid lea. bytes after optional seg prefixes start with 0x8D ∧ ((modrm>>3)&7)=reg_num(dst)
- Test file: src/backend/i686/assembler/encoder/encode_lea_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_lea
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mem, dst]
  relation:
    op: holds
    expr: "strip_seg(bytes)[0] == 0x8D && ((modrm >> 3) & 7) == dst_num"
generators:
  dst: { gen: string, type: "&str" }
evidence: gp_integer.rs:339-340
```

## encode_lea_meta_vs_movl_modrm
- Tier: 4
- Rationale: Metamorphic — LEA and MOV mem→reg (movl) share Mod+RM/SIB/disp for the same memory operand; only opcode differs (8D vs 8B).
- Doc contract: gp_integer.rs:332 vs encode_mov_mem_reg — other fingerprint 228d9fdf
- Seed: encode_invlpg metamorphic vs lidt
- Formal: ∀ mem, dst. tail(encode_lea(mem,dst)) = tail(encode_movl(mem,dst)) where tail drops the single opcode byte
- Test file: src/backend/i686/assembler/encoder/encode_lea_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_lea
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mem, dst]
  relation:
    op: eq
    lhs: "encode_lea(mem,dst)[1..]"
    rhs: "encode_movl(mem,dst)[1..]"
generators:
  mem: { gen: string, type: "MemoryOperand" }
evidence: both call encode_modrm_mem with same reg_field=dst
```

## encode_lea_neg_arity_and_shape
- Tier: 3
- Rationale: Negative contract — wrong arity and non-(Memory,Register) shapes must Err with documented messages.
- Doc contract: gp_integer.rs:333-334 "lea requires 2 operands"; gp_integer.rs:342 "lea requires memory source and register destination" — asserted fingerprint leaerr01
- Seed: (none)
- Formal: ∀ ops. |ops|≠2 ∨ shape≠(Mem,Reg) ⇒ encode_lea(ops)=Err
- Test file: src/backend/i686/assembler/encoder/encode_lea_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_lea
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  relation:
    op: throws
    expr: "sut_encode(\"leal\", ops)"
generators:
  arity: { gen: int, min: 0, max: 3, type: usize }
expected_error: "lea requires"
evidence: gp_integer.rs:333-343
```

## encode_lea_neg_non_gp_and_r8_dest
- Tier: 3
- Rationale: LEA destination is r16/r32 GP only (Intel SDM). reg_num aliases xmm/mm/st and r8 names; SUT must reject non-GP and r8 dest (llvm-mc rejects). Also r16 dest via leal is invalid without 0x66/leaw.
- Doc contract: (none on function) — inferred from Intel SDM LEA r16/r32,m + llvm-mc rejection
- Seed: encode_movzx_neg_non_gp
- Formal: ∀ bad_dst ∈ NON_GP ∪ R8 ∪ R16. encode_lea(mem, bad_dst)=Err under mnemonic leal
- Test file: src/backend/i686/assembler/encoder/encode_lea_pbt.rs
- Status: failing
- Counterexample: mode=4, dst=xmm0; leal (%eax), %xmm0 → sut=[0x8d,0x00]
- Bug report: pbt-out/bug_reports/encode_lea_accepts_non_gp_dest.md
- Re-verified: cargo test --lib encode_lea_neg_shape_and_dest -- --test-threads=1 → FAIL (serial)

```property
function: encode_lea
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad_dst]
  relation:
    op: throws
    expr: "sut_encode(\"leal\", [mem, Register(bad_dst)])"
generators:
  bad_dst: { gen: string, type: "&str" }
expected_error: "bad dst|lea requires"
evidence: Intel SDM LEA; registers.rs reg_num aliases; llvm-mc
```
