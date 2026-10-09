# Properties: encode_mov_mem_reg (i686)

## encode_mov_mem_reg_diff_llvm_mc_base_disp
- Tier: 4
- Rationale: Strongest oracle is differential vs independent llvm-mc i686 assembler. State machine N/A (pure encoder). Round-trip rejected (no in-tree i686 MOV decoder). Reference KAT gate precedes PBT. Domain: same-width GP dest + base+disp memory, no segment.
- Doc contract: (none) — function has no doc comment; Intel SDM MOV r,m / AT&T movl/movw/movb mem,%reg inferred.
- Seed: encode_invlpg_pbt.rs:293 base_disp differential
- Formal: ∀ base ∈ GP32, disp ∈ i32, width ∈ {1,2,4}, dst ∈ GP(width). bytes(encode_mov_mem_reg(mem(base,disp), dst, width)) = llvm-mc(att_mov(width, mem, dst))
- Test file: src/backend/i686/assembler/encoder/encode_mov_mem_reg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_mem_reg
oracle: differential
predicate:
  quantifier: forall
  vars: [base, disp, width, dst]
  domain: { base: GP32, disp: i32, width: "{1,2,4}", dst: "GP(width)" }
  relation:
    op: eq
    lhs: "sut_encode(suffix(width), Memory(base,disp), Register(dst))"
    rhs: "llvm_mc_bytes(att_asm)"
generators:
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: str }
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
  dst: { gen: oneof, values: ["al","ax","eax"], type: str }
evidence: gp_integer.rs:194-214; Intel SDM MOV 8A/8B; llvm-mc -triple=i686
```

## encode_mov_mem_reg_diff_llvm_mc_sib
- Tier: 4
- Rationale: SIB forms (ESP base, index≠esp, scales 1/2/4/8, optional no-base) are the densest ModR/M edge space; differential vs llvm-mc.
- Doc contract: (none)
- Seed: encode_invlpg_pbt.rs:320 sib differential
- Formal: ∀ base?, index≠esp, scale∈{1,2,4,8}, disp∈i32, width∈{1,2,4}, dst∈GP(width). bytes(SUT mov mem→reg) = llvm-mc(att)
- Test file: src/backend/i686/assembler/encoder/encode_mov_mem_reg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_mem_reg
oracle: differential
predicate:
  quantifier: forall
  vars: [base, index, scale, disp, width, dst]
  domain: { index: "GP32\\{esp}", scale: "{1,2,4,8}", width: "{1,2,4}" }
  relation:
    op: eq
    lhs: sut_bytes
    rhs: llvm_mc_bytes
generators:
  base: { gen: optional, inner: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: str } }
  index: { gen: oneof, values: ["eax","ecx","edx","ebx","ebp","esi","edi"], type: str }
  scale: { gen: oneof, values: [1,2,4,8], type: u8 }
  disp: { gen: int, min: -512, max: 512, type: i64 }
  width: { gen: oneof, values: [1,2,4], type: u8 }
evidence: core.rs:45-143 encode_modrm_mem SIB path
```

## encode_mov_mem_reg_diff_llvm_mc_segment
- Tier: 4
- Rationale: Segment override is a documented i686 encoding contract (core.rs emit_segment_prefix handles es/cs/ss/ds/fs/gs). encode_mov_mem_reg inlines fs/gs-only and errors on others — differential vs llvm-mc covers all six. Failure is a SUT bug, not a domain exclusion.
- Doc contract: (none) — function has no doc comment. Body gp_integer.rs:198-203 fs/gs-only match is the producing statement under test, not a domain-restriction comment. fingerprint 16908055
- Seed: encode_invlpg_pbt.rs:352 segment differential
- Formal: ∀ seg ∈ {es,cs,ss,ds,fs,gs}, base∈GP32, disp∈i32, width∈{1,2,4}, dst∈GP(width). bytes(SUT) = llvm-mc(att with %seg:mem)
- Test file: src/backend/i686/assembler/encoder/encode_mov_mem_reg_pbt.rs
- Status: failing
- Counterexample: seg="es", base="eax", disp=0, width=1, dst=al — asm `movb %es:(%eax), %al`; SUT Err("unsupported segment: es"); llvm-mc [0x26, 0x8a, 0x00]
- Bug report: bug_reports/encode_mov_mem_reg_missing_segment_prefix.md
- Re-verified: PBT_TEST_JOBS=1 cargo test --lib encode_mov_mem_reg_diff_llvm_mc_segment -- --test-threads=1 → FAIL (serial)

```property
function: encode_mov_mem_reg
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, disp, width, dst]
  domain: { seg: "{es,cs,ss,ds,fs,gs}", base: GP32, width: "{1,2,4}" }
  relation:
    op: eq
    lhs: sut_bytes
    rhs: llvm_mc_bytes
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: str }
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: str }
  disp: { gen: oneof, values: [0, 8, -4, 127, -128], type: i64 }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
evidence: core.rs:31-42 emit_segment_prefix; Intel SDM 2.1.1; llvm-mc accepts %es:(%eax)
```

## encode_mov_mem_reg_diff_edges_esp_ebp_abs
- Tier: 4
- Rationale: Boundary ModR/M forms (ESP→SIB, EBP→forced disp, disp8/disp32 edges, absolute no-base). Skip only when llvm-mc chooses moffs A0/A1 for abs→eAX (encoding choice; both valid).
- Doc contract: (none)
- Seed: encode_invlpg_pbt.rs:375 edges
- Formal: ∀ edge ∈ ESP/EBP/SIB/abs (excluding moffs-preferred abs→eAX). bytes(SUT) = llvm-mc(att)
- Test file: src/backend/i686/assembler/encoder/encode_mov_mem_reg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_mem_reg
oracle: differential
predicate:
  quantifier: forall
  vars: [edge, width, dst]
  domain: { edge: "0..14", width: "{1,2,4}" }
  relation:
    op: eq
    lhs: sut_bytes
    rhs: llvm_mc_bytes
generators:
  edge: { gen: int, min: 0, max: 13, type: u8 }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
  dst: { gen: oneof, values: ["al","ax","eax","bl","bx","ebx"], type: str }
evidence: core.rs:69-140 special ESP/EBP/abs encodings
```

## encode_mov_mem_reg_invariant_opcode_modrm
- Tier: 3
- Rationale: Algebraic invariant independent of llvm-mc — after optional 0x66, opcode is 8A (size1) or 8B (else), ModRM.reg = dst_num.
- Doc contract: (none)
- Seed: encode_invlpg_pbt.rs invariant_opcode_ext7
- Formal: ∀ valid same-width mem→reg. let b = bytes after optional 0x66. b[0]∈{0x8A,0x8B} ∧ ((b[1]>>3)&7) = reg_num(dst)
- Test file: src/backend/i686/assembler/encoder/encode_mov_mem_reg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_mem_reg
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mem, dst, width]
  domain: { width: "{1,2,4}", dst: "GP(width)" }
  relation:
    op: holds
    expr: "opcode_in_{8A,8B}(sut) && modrm_reg(sut) == gp_num(dst)"
generators:
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: str }
  disp: { gen: int, min: -200, max: 300, type: i64 }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
evidence: gp_integer.rs:206-213; Intel SDM MOV
```

## encode_mov_mem_reg_meta_load_vs_store_modrm
- Tier: 3
- Rationale: Metamorphic — same memory + same GP, load (8A/8B) and store (88/89) share segment/66/Mod+RM/SIB/disp; only opcode byte differs. Required metamorphic angle for standard tier.
- Doc contract: (none)
- Seed: encode_invlpg_pbt.rs meta vs lidt
- Formal: ∀ mem, reg, width∈{1,2,4}. prefixes(load)=prefixes(store) ∧ tail(load)=tail(store) ∧ load_op∈{8A,8B} ∧ store_op∈{88,89}
- Test file: src/backend/i686/assembler/encoder/encode_mov_mem_reg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_mem_reg
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mem, reg, width]
  domain: { width: "{1,2,4}", reg: "GP(width)", seg: "{None,fs}" }
  relation:
    op: eq
    lhs: "modrm_tail(encode_mov_mem_reg(mem,reg,w))"
    rhs: "modrm_tail(encode_mov_reg_mem(reg,mem,w))"
generators:
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: str }
  disp: { gen: int, min: -1000, max: 1000, type: i64 }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
  use_fs: { gen: bool }
evidence: gp_integer.rs:194-236 load/store pair
```

## encode_mov_mem_reg_neg_mismatched_width
- Tier: 3
- Rationale: Negative/error — llvm-mc rejects mismatched dest width (movl mem, %ax). SUT must not silently emit wrong-size opcode via reg_num aliasing. Contract inferred from Intel MOV width pairing + llvm-mc.
- Doc contract: (none) — no width gate in body; reg_num aliases ax→0 like eax
- Seed: encode_mov_rr_pbt mismatched width
- Formal: ∀ width, dst with reg_size(dst)≠width, valid mem. encode_mov_mem_reg(...) = Err(_)
- Test file: src/backend/i686/assembler/encoder/encode_mov_mem_reg_pbt.rs
- Status: failing
- Counterexample: mnemonic=movl, mem=(%eax), dst=ax → Ok([0x8b, 0x00]); llvm-mc rejects
- Bug report: bug_reports/encode_mov_mem_reg_mismatched_width.md
- Re-verified: cargo test --lib encode_mov_mem_reg_neg_mismatched_width -- --test-threads=1 → FAIL (serial, first full run)

```property
function: encode_mov_mem_reg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [width, dst_wrong, mem]
  domain: { width: "{1,2,4}", dst_wrong: "GP \\ GP(width)" }
  relation:
    op: holds
    expr: "sut_encode(suffix(width), mem, dst_wrong).is_err()"
generators:
  mode: { gen: int, min: 0, max: 5, type: u8 }
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: str }
  disp: { gen: oneof, values: [0, 4, -8], type: i64 }
expected_error: String
evidence: llvm-mc rejects movl (%eax), %ax; Intel MOV r/m width match
```

## encode_mov_mem_reg_neg_non_gp_dest
- Tier: 2
- Rationale: Non-GP dest names that alias through reg_num (xmm/mm/st) must Err; 8A/8B is GP-only. llvm-mc rejects.
- Doc contract: (none)
- Seed: encode_mov_rr_pbt non_gp
- Formal: ∀ bad ∈ {xmm0,mm0,st,…}, mem, width. encode path yields Err
- Test file: src/backend/i686/assembler/encoder/encode_mov_mem_reg_pbt.rs
- Status: failing
- Counterexample: mnemonic=movb, mem=(%eax), dst=xmm0 → Ok([0x8a, 0x00]); llvm-mc rejects
- Bug report: bug_reports/encode_mov_mem_reg_non_gp_dest.md
- Re-verified: cargo test --lib encode_mov_mem_reg_neg_non_gp_dest -- --test-threads=1 → FAIL (serial, first full run)

```property
function: encode_mov_mem_reg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad_dst, mem, width]
  domain: { bad_dst: "{xmm0,mm0,st,...}", width: "{1,2,4}" }
  relation:
    op: holds
    expr: "sut_encode(suffix(width), mem, bad_dst).is_err()"
generators:
  ni: { gen: int, min: 0, max: 9, type: usize }
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: str }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
expected_error: String
evidence: registers.rs:4-15 reg_num; llvm-mc rejects movl (%eax), %xmm0
```
