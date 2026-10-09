# Properties: encode_mov_seg (i686)

## encode_mov_seg_diff_sreg_to_gp32
- Tier: 4
- Rationale: Strongest oracle is differential vs llvm-mc (independent assembler). State machine N/A (pure encode). Round-trip N/A (no decoder). Evidence: system.rs:273-322; Intel SDM MOV Sreg; llvm-mc `movl %ds, %eax` → `[8c,d8]`.
- Doc contract: system.rs:273 "Encode MOV to/from segment register" — asserted fingerprint f66b8f37
- Seed: (none) — generalized from sibling encode_mov_cr_pbt KAT pattern
- Formal: ∀ sreg ∈ {es,cs,ss,ds,fs,gs}, gp ∈ GP32. encode_mov_seg(movl, sreg, gp) = llvm_mc("movl %sreg, %gp")
- Test file: src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_seg
oracle: differential
predicate:
  quantifier: forall
  vars: [sreg, gp]
  domain: { sreg: segment_regs, gp: gp32 }
  relation:
    op: eq
    lhs: "sut_encode(movl, [sreg, gp])"
    rhs: "llvm_mc(movl %sreg, %gp)"
generators:
  sreg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
  gp: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: system.rs:293-298
```

## encode_mov_seg_diff_gp32_to_sreg
- Tier: 4
- Rationale: Differential write direction (8E /r). Same evidence chain as read form.
- Doc contract: system.rs:273 "Encode MOV to/from segment register" — asserted fingerprint f66b8f37
- Seed: (none)
- Formal: ∀ gp ∈ GP32, sreg ∈ SEG. encode_mov_seg(movl, gp, sreg) = llvm_mc("movl %gp, %sreg")
- Test file: src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_seg
oracle: differential
predicate:
  quantifier: forall
  vars: [gp, sreg]
  domain: { gp: gp32, sreg: segment_regs }
  relation:
    op: eq
    lhs: "sut_encode(movl, [gp, sreg])"
    rhs: "llvm_mc(movl %gp, %sreg)"
generators:
  gp: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  sreg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
evidence: system.rs:301-306
```

## encode_mov_seg_diff_sreg_to_r16
- Tier: 4
- Rationale: Differential for 16-bit dest: llvm-mc emits 0x66 operand-size prefix (`movw %ds, %ax` → `[66,8c,d8]`). encode_mov_seg ignores size once routed — misses 0x66.
- Doc contract: system.rs:273 "Encode MOV to/from segment register" — asserted fingerprint f66b8f37
- Seed: (none)
- Formal: ∀ sreg ∈ SEG, r16 ∈ R16. encode_mov_seg(movw, sreg, r16) = llvm_mc("movw %sreg, %r16")
- Test file: src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs
- Status: failing
- Counterexample: movw %es, %ax → SUT [8c,c0] vs llvm-mc [66,8c,c0]
- Bug report: bug_reports/encode_mov_seg_missing_66_r16.md

```property
function: encode_mov_seg
oracle: differential
predicate:
  quantifier: forall
  vars: [sreg, r16]
  domain: { sreg: segment_regs, r16: r16 }
  relation:
    op: eq
    lhs: "sut_encode(movw, [sreg, r16])"
    rhs: "llvm_mc(movw %sreg, %r16)"
generators:
  sreg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
  r16: { gen: oneof, values: ["ax","cx","dx","bx","sp","bp","si","di"] }
evidence: system.rs:293-298; llvm-mc movw %ds,%ax = [66,8c,d8]
```

## encode_mov_seg_diff_r16_to_sreg
- Tier: 4
- Rationale: Write to sreg from r16; llvm-mc encodes without 0x66 (`movw %ax, %ds` → `[8e,d8]`).
- Doc contract: system.rs:273 "Encode MOV to/from segment register" — asserted fingerprint f66b8f37
- Seed: (none)
- Formal: ∀ r16 ∈ R16, sreg ∈ SEG. encode_mov_seg(movw, r16, sreg) = llvm_mc("movw %r16, %sreg")
- Test file: src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_seg
oracle: differential
predicate:
  quantifier: forall
  vars: [r16, sreg]
  domain: { r16: r16, sreg: segment_regs }
  relation:
    op: eq
    lhs: "sut_encode(movw, [r16, sreg])"
    rhs: "llvm_mc(movw %r16, %sreg)"
generators:
  r16: { gen: oneof, values: ["ax","cx","dx","bx","sp","bp","si","di"] }
  sreg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
evidence: system.rs:301-306
```

## encode_mov_seg_diff_mem
- Tier: 4
- Rationale: Memory forms use 8C/8E + ModR/M; llvm-mc requires movw for mem. Covers base/disp/SIB/abs without segment override.
- Doc contract: system.rs:273 "Encode MOV to/from segment register" — asserted fingerprint f66b8f37
- Seed: encode_smsw_pbt.rs memory differential pattern
- Formal: ∀ sreg ∈ SEG, mem ∈ MemNoSeg, dir ∈ {store,load}. encode_mov_seg(movw, sreg↔mem) = llvm_mc(movw AT&T form)
- Test file: src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_seg
oracle: differential
predicate:
  quantifier: forall
  vars: [sreg, mem, dir]
  domain: { sreg: segment_regs, mem: mem_no_seg, dir: {store,load} }
  relation:
    op: eq
    lhs: "sut_encode(movw, sreg_mem_ops)"
    rhs: "llvm_mc(movw att)"
generators:
  sreg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
  mem: { gen: mem_forms_no_seg }
evidence: system.rs:309-319
```

## encode_mov_seg_diff_mem_segment
- Tier: 4
- Rationale: Memory with segment override must emit 0x26/0x2E/0x36/0x3E/0x64/0x65 before opcode (core.rs:31-42 emit_segment_prefix). Sibling system encoders omit this — confirmed fail.
- Doc contract: system.rs:273 "Encode MOV to/from segment register" — asserted fingerprint f66b8f37
- Seed: encode_lmsw_pbt / encode_invlpg_pbt segment differential
- Formal: ∀ sreg ∈ SEG, mem ∈ MemWithSeg, dir ∈ {store,load}. encode_mov_seg(movw, …) = llvm_mc including segment prefix
- Test file: src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs
- Status: failing
- Counterexample: movw %es:(%eax), %es → SUT [8e,00] vs llvm-mc [26,8e,00]
- Bug report: bug_reports/encode_mov_seg_missing_segment_prefix.md

```property
function: encode_mov_seg
oracle: differential
predicate:
  quantifier: forall
  vars: [sreg, mem, dir]
  domain: { sreg: segment_regs, mem: mem_with_seg, dir: {store,load} }
  relation:
    op: eq
    lhs: "sut_encode(movw, sreg_mem_ops)"
    rhs: "llvm_mc(movw %seg:…)"
generators:
  sreg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
  mem: { gen: mem_forms_with_seg }
evidence: system.rs:309-319; core.rs:31-42
```

## encode_mov_seg_diff_segment_sib
- Tier: 4
- Rationale: Strengthening — SIB addressing plus segment override must still emit the override prefix (same root cause as mem_segment).
- Doc contract: system.rs:273 "Encode MOV to/from segment register" — asserted fingerprint f66b8f37
- Seed: encode_lmsw_pbt segment+SIB
- Formal: ∀ sreg, mseg, base, index≠esp, scale, dir. encode(movw, SIB+seg) = llvm_mc
- Test file: src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs
- Status: failing
- Counterexample: movw %es:4(%eax,%eax,1), %es → SUT [8e,44,00,04] vs llvm-mc [26,8e,44,00,04]
- Bug report: bug_reports/encode_mov_seg_missing_segment_prefix_sib.md

```property
function: encode_mov_seg
oracle: differential
predicate:
  quantifier: forall
  vars: [sreg, mseg, base, index, scale, dir]
  domain: { sreg: segment_regs, mseg: segment_regs, base: gp32, index: gp32_no_esp, scale: scales, dir: store_or_load }
  relation:
    op: eq
    lhs: "sut_encode(movw, sib_seg_ops)"
    rhs: "llvm_mc(movw att)"
generators:
  sreg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
  mseg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
evidence: system.rs:309-319; core.rs:31-42
```

## encode_mov_seg_invariant_opcode_modrm
- Tier: 3
- Rationale: Algebraic invariant — register form is exactly 2 bytes: opc ∈ {0x8C,0x8E}, ModRM mod=3, reg=sreg#, rm=gp#.
- Doc contract: system.rs:273 "Encode MOV to/from segment register" — asserted fingerprint f66b8f37
- Seed: encode_mov_cr_pbt invariant
- Formal: ∀ write ∈ Bool, sreg, gp32. bytes = encode(movl,…) ⇒ |bytes|=2 ∧ bytes[0]=8C|8E ∧ mod=3 ∧ reg=sreg_num ∧ rm=gp_num
- Test file: src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_seg
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [write, sreg, gp]
  domain: { write: bool, sreg: segment_regs, gp: gp32 }
  body: "let b = sut_encode(movl, ops(write,sreg,gp)); b.len()==2 && b[0]==(if write {0x8E} else {0x8C}) && (b[1]>>6)==3 && ((b[1]>>3)&7)==seg_num(sreg) && (b[1]&7)==gp_num(gp)"
generators:
  write: { gen: bool }
  sreg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
  gp: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: system.rs:293-306
```

## encode_mov_seg_metamorphic_read_write
- Tier: 3
- Rationale: Required metamorphic — same sreg/gp pair, read (8C) and write (8E) share ModRM; only opcode differs.
- Doc contract: system.rs:273 "Encode MOV to/from segment register" — asserted fingerprint f66b8f37
- Seed: encode_mov_cr_pbt metamorphic
- Formal: ∀ sreg, gp. encode(sreg→gp)[1] = encode(gp→sreg)[1] ∧ opc_read=0x8C ∧ opc_write=0x8E
- Test file: src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_seg
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [sreg, gp]
  domain: { sreg: segment_regs, gp: gp32 }
  relation:
    op: eq
    lhs: "encode(sreg,gp)[1]"
    rhs: "encode(gp,sreg)[1]"
generators:
  sreg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
  gp: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: system.rs:293-306
```

## encode_mov_seg_neg_arity
- Tier: 3
- Rationale: Negative/error — wrong arity must Err with "2 operand" message.
- Doc contract: system.rs:273 "Encode MOV to/from segment register" — asserted fingerprint f66b8f37
- Seed: encode_mov_cr_pbt neg
- Formal: ∀ n≠2. encode_mov_seg(n ops including a segment) = Err containing "2 operand"
- Test file: src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_seg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: arity_not_2 }
  body: "sut_encode(movl, ops_of_len(n)).is_err()"
generators:
  n: { gen: int, min: 0, max: 4 }
expected_error: String
evidence: system.rs:275-277
```

## encode_mov_seg_neg_r8
- Tier: 3
- Rationale: Negative/error — r8 GP must be rejected (Intel MOV Sreg is r/m16 or r32, not r8; llvm-mc rejects).
- Doc contract: system.rs:273 "Encode MOV to/from segment register" — asserted fingerprint f66b8f37
- Seed: encode_mov_cr_pbt neg
- Formal: ∀ r8,sreg,dir. encode(movl with r8) = Err
- Test file: src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs
- Status: failing
- Counterexample: movl %es, %al → Ok([8c,c0]); llvm-mc rejects
- Bug report: bug_reports/encode_mov_seg_accepts_r8.md

```property
function: encode_mov_seg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [sreg, r8, to_sreg]
  domain: { sreg: segment_regs, r8: r8, to_sreg: bool }
  body: "sut_encode(movl, r8_ops(sreg,r8,to_sreg)).is_err()"
generators:
  sreg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
  r8: { gen: oneof, values: ["al","cl","dl","bl","ah","ch","dh","bh"] }
  to_sreg: { gen: bool }
expected_error: String
evidence: system.rs:293-306; Intel SDM MOV Sreg r/m16
```

## encode_mov_seg_diff_mnemonic_aliases
- Tier: 4
- Rationale: Strengthening — unsuffixed `mov` must agree with `movl` / llvm-mc for r32 forms.
- Doc contract: system.rs:273 "Encode MOV to/from segment register" — asserted fingerprint f66b8f37
- Seed: encode_mov_cr_pbt mnemonic aliases
- Formal: ∀ write,sreg,gp. encode(mov,…) = encode(movl,…) = llvm_mc(movl …)
- Test file: src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_seg
oracle: differential
predicate:
  quantifier: forall
  vars: [write, sreg, gp]
  domain: { write: bool, sreg: segment_regs, gp: gp32 }
  relation:
    op: eq
    lhs: "sut_encode(mov, ops)"
    rhs: "llvm_mc(movl att)"
generators:
  write: { gen: bool }
  sreg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
  gp: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: gp_integer.rs:21-23; mod.rs mov dispatch
```
