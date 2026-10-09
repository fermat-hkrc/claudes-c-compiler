# Properties: encode_mov_cr (i686)

## encode_mov_cr_diff_cr_to_gp
- Tier: 5
- Rationale: Strongest oracle is differential vs llvm-mc (independent assembler). State machine N/A (pure encode). Round-trip N/A (no i686 CR decoder). Intel/AT&T MOV CR→GP is 0F 20 /r.
- Doc contract: system.rs:249 "Encode MOV to/from control register: 0F 20 /r (read) or 0F 22 /r (write)" — asserted fingerprint 8d63ee6c
- Seed: (none — no prior mov-cr unit tests)
- Formal: ∀ cr ∈ {cr0,cr2,cr3,cr4}, gp ∈ GP32. encode(movl %cr, %gp) = llvm_mc("movl %cr, %gp")
- Test file: src/backend/i686/assembler/encoder/encode_mov_cr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_cr
oracle: differential
predicate:
  quantifier: forall
  vars: [cr, gp]
  domain: { cr: control_regs, gp: gp32 }
  relation:
    op: eq
    lhs: "sut_encode(movl, [Reg(cr), Reg(gp)])"
    rhs: "llvm_mc(movl %cr, %gp)"
generators:
  cr: { gen: oneof, values: ["cr0", "cr2", "cr3", "cr4"] }
  gp: { gen: oneof, values: ["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"] }
evidence: system.rs:249-260; Intel SDM MOV CR; llvm-mc i686
```

## encode_mov_cr_diff_gp_to_cr
- Tier: 5
- Rationale: Symmetric write direction 0F 22 /r; same differential reference.
- Doc contract: system.rs:249 "Encode MOV to/from control register: 0F 20 /r (read) or 0F 22 /r (write)" — asserted fingerprint 8d63ee6c
- Seed: (none)
- Formal: ∀ cr ∈ CR, gp ∈ GP32. encode(movl %gp, %cr) = llvm_mc("movl %gp, %cr")
- Test file: src/backend/i686/assembler/encoder/encode_mov_cr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_cr
oracle: differential
predicate:
  quantifier: forall
  vars: [gp, cr]
  domain: { gp: gp32, cr: control_regs }
  relation:
    op: eq
    lhs: "sut_encode(movl, [Reg(gp), Reg(cr)])"
    rhs: "llvm_mc(movl %gp, %cr)"
generators:
  gp: { gen: oneof, values: ["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"] }
  cr: { gen: oneof, values: ["cr0", "cr2", "cr3", "cr4"] }
evidence: system.rs:249-267
```

## encode_mov_cr_invariant_opcode_modrm
- Tier: 4
- Rationale: Algebraic invariant from Intel encoding: bytes = [0F, 20|22, modrm(3, cr_num, gp_num)].
- Doc contract: system.rs:249 "Encode MOV to/from control register: 0F 20 /r (read) or 0F 22 /r (write)" — asserted fingerprint 8d63ee6c
- Seed: (none)
- Formal: ∀ dir, cr, gp. let b = encode(...). b = [0x0F, opc, m] ∧ opc∈{0x20,0x22} ∧ (m>>6)=3 ∧ ((m>>3)&7)=cr_num(cr) ∧ (m&7)=reg_num(gp)
- Test file: src/backend/i686/assembler/encoder/encode_mov_cr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_cr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [dir, cr, gp]
  domain: { dir: {read,write}, cr: control_regs, gp: gp32 }
  relation:
    op: holds
    expr: "bytes==[0x0F, opc, modrm] && opc in {0x20,0x22} && mod==3 && reg==cr_num && rm==gp_num"
generators:
  dir: { gen: bool }
  cr: { gen: oneof, values: ["cr0", "cr2", "cr3", "cr4"] }
  gp: { gen: oneof, values: ["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"] }
evidence: system.rs:249-267; Intel SDM
```

## encode_mov_cr_metamorphic_read_write
- Tier: 4
- Rationale: Required metamorphic: same CR/GP pair — read and write encodings share ModRM; only opcode byte differs (0x20 vs 0x22).
- Doc contract: system.rs:249 "Encode MOV to/from control register: 0F 20 /r (read) or 0F 22 /r (write)" — asserted fingerprint 8d63ee6c
- Seed: (none)
- Formal: ∀ cr, gp. encode(cr→gp)[0]=encode(gp→cr)[0]=0x0F ∧ encode(cr→gp)[1]=0x20 ∧ encode(gp→cr)[1]=0x22 ∧ encode(cr→gp)[2]=encode(gp→cr)[2]
- Test file: src/backend/i686/assembler/encoder/encode_mov_cr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_cr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [cr, gp]
  domain: { cr: control_regs, gp: gp32 }
  relation:
    op: holds
    expr: "read[0]==write[0]==0x0F && read[1]==0x20 && write[1]==0x22 && read[2]==write[2]"
generators:
  cr: { gen: oneof, values: ["cr0", "cr2", "cr3", "cr4"] }
  gp: { gen: oneof, values: ["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"] }
evidence: system.rs:255-267
```

## encode_mov_cr_neg_arity
- Tier: 3
- Rationale: Negative contract — arity must be exactly 2.
- Doc contract: system.rs:251-253 if ops.len() != 2 return Err("mov cr requires 2 operands") — domain-restriction fingerprint a1b2c3d4
- Seed: (none)
- Formal: ∀ ops. len(ops)≠2 ⇒ encode path for movl with CR-shaped ops yields Err mentioning 2 operands
- Test file: src/backend/i686/assembler/encoder/encode_mov_cr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_cr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: arity != 2 }
  relation:
    op: holds
    expr: "sut_encode(movl, ops).is_err()"
expected_error: "mov cr requires 2 operands"
generators:
  n: { gen: int, min: 0, max: 5 }
evidence: system.rs:251-253
```

## encode_mov_cr_neg_bad_operands
- Tier: 3
- Rationale: Non CR↔r32 pairs must Err. 8/16-bit GP rejected by llvm-mc (Intel r32 only). Found: SUT accepts r16/r8 via reg_num aliasing.
- Doc contract: system.rs:249 "Encode MOV to/from control register: 0F 20 /r (read) or 0F 22 /r (write)" — asserted fingerprint 8d63ee6c
- Seed: encode_lmsw_pbt.rs non-r16 rejection pattern
- Formal: ∀ bad ∈ {r16,r8,...}. llvm_mc rejects ⇒ SUT Err (no silent encode as r32)
- Test file: src/backend/i686/assembler/encoder/encode_mov_cr_pbt.rs
- Status: failing
- Counterexample: movl %cr0, %ax → Ok([0x0f, 0x20, 0xc0]); also movl %al, %cr0 → Ok([0x0f, 0x22, 0xc0])
- Bug report: pbt-out/bug_reports/encode_mov_cr_non_r32_gp.md

```property
function: encode_mov_cr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, bad]
  domain: { kind: imm|mem|label|r16|r8|seg }
  relation:
    op: holds
    expr: "sut_rejects_or_matches_llvm_mc_reject(kind, bad)"
expected_error: unsupported mov cr operands
generators:
  kind: { gen: int, min: 0, max: 7 }
evidence: system.rs:269; Intel MOV CR r32; llvm-mc rejects r16/r8
```

## encode_mov_cr_diff_mnemonic_aliases
- Tier: 5
- Rationale: Strengthen — unsuffixed `mov` and `movl` must agree with llvm-mc for valid CR↔r32 forms.
- Doc contract: system.rs:249 "Encode MOV to/from control register: 0F 20 /r (read) or 0F 22 /r (write)" — asserted fingerprint 8d63ee6c
- Seed: (none)
- Formal: ∀ cr, gp, dir. encode(mov, ...) = encode(movl, ...) = llvm_mc(movl ...)
- Test file: src/backend/i686/assembler/encoder/encode_mov_cr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_cr
oracle: differential
predicate:
  quantifier: forall
  vars: [cr, gp, dir]
  domain: { cr: control_regs, gp: gp32, dir: {r,w} }
  relation:
    op: eq
    lhs: "sut_encode(mov, ops)"
    rhs: "llvm_mc(movl ...)"
generators:
  cr: { gen: oneof, values: ["cr0", "cr2", "cr3", "cr4"] }
  gp: { gen: oneof, values: ["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"] }
evidence: mod.rs:151-155; gp_integer.rs:18-19
```

## encode_mov_cr_neg_movw_width
- Tier: 3
- Rationale: Strengthening round — `movw` with CR must not emit 0F 20/22 (Intel r32-only; llvm-mc rejects). Same root-cause class as non-r32 GP acceptance.
- Doc contract: system.rs:249 "Encode MOV to/from control register: 0F 20 /r (read) or 0F 22 /r (write)" — asserted fingerprint 8d63ee6c
- Seed: (none)
- Formal: ∀ cr, r16, dir. llvm_mc rejects movw CR form ⇒ SUT Err
- Test file: src/backend/i686/assembler/encoder/encode_mov_cr_pbt.rs
- Status: failing
- Counterexample: movw %cr0, %ax → Ok([0x0f, 0x20, 0xc0])
- Bug report: pbt-out/bug_reports/encode_mov_cr_movw_accepted.md

```property
function: encode_mov_cr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [cr, r16, write]
  domain: { cr: control_regs, r16: gp16 }
  relation:
    op: holds
    expr: "sut_encode(movw, ops).is_err()"
expected_error: width / unsupported
generators:
  cr: { gen: oneof, values: ["cr0", "cr2", "cr3", "cr4"] }
  r16: { gen: oneof, values: ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"] }
evidence: Intel SDM MOV CR r32; llvm-mc rejects movw %cr0, %ax
```
