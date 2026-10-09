# Properties: encode_mov_infer_size

## encode_mov_infer_size_diff_rr_same_width
- Tier: 4
- Rationale: Strongest oracle is differential vs llvm-mc (independent assembler claiming the same AT&T i686 encoding). State machine N/A (pure encoder). Round-trip N/A (no decoder). Same-width GP RR is the core size-inference path (first-register `reg_size`).
- Doc contract: gp_integer.rs:111 "Handle unsuffixed `mov` from inline asm - infer size from operands" — asserted fingerprint 7b2e4c91
- Seed: encode_mov_cr_pbt.rs:321 (unsuffixed mov alias) generalized to GP RR
- Formal: ∀ src,dst ∈ GP_w (w∈{1,2,4}). encode("mov",[src,dst]) = llvm_mc("mov %src, %dst")
- Test file: src/backend/i686/assembler/encoder/encode_mov_infer_size_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_infer_size
oracle: differential
predicate:
  quantifier: forall
  vars: [src, dst, width]
  domain: { src: gp_reg(width), dst: gp_reg(width), width: {1,2,4} }
  relation:
    op: eq
    lhs: "sut_encode(\"mov\", [Reg(src), Reg(dst)])"
    rhs: "llvm_mc_bytes(format!(\"mov %{src}, %{dst}\"))"
generators:
  width: { gen: oneof, choices: [1, 2, 4], type: u8 }
  src: { gen: string, type: String }
  dst: { gen: string, type: String }
evidence: gp_integer.rs:112-123; mod.rs:163; llvm-mc -triple=i686
```

## encode_mov_infer_size_diff_imm_reg
- Tier: 4
- Rationale: Imm→Reg size must come from the destination register (second operand), not the default-4 branch. Differential vs llvm-mc.
- Doc contract: gp_integer.rs:111 "Handle unsuffixed `mov` from inline asm - infer size from operands" — asserted fingerprint 7b2e4c91
- Seed: (none)
- Formal: ∀ imm ∈ Int, dst ∈ GP_w (w∈{1,2,4}). encode("mov",[Imm(imm),Reg(dst)]) = llvm_mc("mov $imm, %dst")
- Test file: src/backend/i686/assembler/encoder/encode_mov_infer_size_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_infer_size
oracle: differential
predicate:
  quantifier: forall
  vars: [imm, dst, width]
  domain: { imm: int_fitting(width), dst: gp_reg(width), width: {1,2,4} }
  relation:
    op: eq
    lhs: "sut_encode(\"mov\", [Imm(imm), Reg(dst)])"
    rhs: "llvm_mc_bytes(format!(\"mov $imm, %dst\"))"
generators:
  width: { gen: oneof, choices: [1, 2, 4], type: u8 }
  imm: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
  dst: { gen: string, type: String }
evidence: gp_integer.rs:117-122; encode_mov_imm_reg
```

## encode_mov_infer_size_diff_mem_reg
- Tier: 4
- Rationale: Mem→Reg and Reg→Mem infer size from the GP register operand; must match llvm-mc including operand-size override 0x66 for r16 and 8-bit opcodes for r8.
- Doc contract: gp_integer.rs:111 "Handle unsuffixed `mov` from inline asm - infer size from operands" — asserted fingerprint 7b2e4c91
- Seed: (none)
- Formal: ∀ mem ∈ MemSimple, r ∈ GP_w. encode("mov",[Mem,r]) = llvm_mc(...) ∧ encode("mov",[r,Mem]) = llvm_mc(...)
- Test file: src/backend/i686/assembler/encoder/encode_mov_infer_size_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_infer_size
oracle: differential
predicate:
  quantifier: forall
  vars: [mem, reg, width, direction]
  domain: { mem: base_disp_mem, reg: gp_reg(width), width: {1,2,4}, direction: {load,store} }
  relation:
    op: eq
    lhs: "sut_encode(\"mov\", ops(direction))"
    rhs: "llvm_mc_bytes(att(direction))"
generators:
  width: { gen: oneof, choices: [1, 2, 4], type: u8 }
  direction: { gen: bool, type: bool }
evidence: gp_integer.rs:117-122
```

## encode_mov_infer_size_metamorphic_suffix
- Tier: 4
- Rationale: Required metamorphic/differential angle — when size is unambiguous from a register, unsuffixed `mov` must emit identical bytes to the size-suffixed mnemonic (`movb`/`movw`/`movl`) and to llvm-mc. Algebraic.metamorphic under size-preserving rename of mnemonic.
- Doc contract: gp_integer.rs:111 "Handle unsuffixed `mov` from inline asm - infer size from operands" — asserted fingerprint 7b2e4c91
- Seed: encode_mov_cr_pbt.rs:321
- Formal: ∀ ops with unambiguous size w. encode("mov",ops) = encode(suffix(w),ops) = llvm_mc(suffix(w) asm)
- Test file: src/backend/i686/assembler/encoder/encode_mov_infer_size_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_infer_size
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [ops, width]
  domain: { ops: unambiguous_mov_ops(width), width: {1,2,4} }
  relation:
    op: eq
    lhs: "sut_encode(\"mov\", ops)"
    rhs: "sut_encode(suffix(width), ops)"
generators:
  width: { gen: oneof, choices: [1, 2, 4], type: u8 }
evidence: mod.rs:163 vs movl/movw/movb dispatch; gp_integer.rs:112-123
```

## encode_mov_infer_size_invariant_inferred_size
- Tier: 3
- Rationale: Structural invariant of the documented inference rule: first Register's reg_size wins; else second; else 4. Cross-check by comparing `mov` bytes to `encode_mov` driven through the matching suffix.
- Doc contract: gp_integer.rs:111-122 match arms — asserted fingerprint 7b2e4c91
- Seed: (none)
- Formal: ∀ ops. let s = infer(ops) in encode("mov",ops) = encode(suffix(s),ops) when suffix path accepts ops
- Test file: src/backend/i686/assembler/encoder/encode_mov_infer_size_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_infer_size
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: mov_two_ops_with_at_least_one_gp }
  body: "sut_encode(\"mov\",ops) == sut_encode(suffix(infer(ops)),ops)"
generators:
  ops: { gen: custom }
evidence: gp_integer.rs:117-122
```

## encode_mov_infer_size_neg_arity
- Tier: 3
- Rationale: Documented arity contract — mov requires exactly 2 operands (shared with encode_mov).
- Doc contract: gp_integer.rs:113-115 "mov requires 2 operands" — asserted fingerprint a1c9d2e0
- Seed: encode_mov_cr_pbt.rs encode_mov_cr_neg_arity
- Formal: ∀ n≠2. encode("mov", ops_n).is_err() ∧ err contains "2 operand"
- Test file: src/backend/i686/assembler/encoder/encode_mov_infer_size_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_infer_size
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: 0..5, n != 2 }
  relation:
    op: throws
    expr: "sut_encode(\"mov\", ops_of_len(n))"
    error: "mov requires 2 operands"
generators:
  n: { gen: int, min: 0, max: 4, type: usize }
expected_error: "mov requires 2 operands"
evidence: gp_integer.rs:113-115
```

## encode_mov_infer_size_neg_ambiguous_imm_mem
- Tier: 4
- Rationale: GAS/llvm-mc reject unsuffixed `mov $imm, mem` as ambiguous (could be b/w/l). The documented job is size inference from operands; with no register operand the default-to-4 path silently invents movl. Negative contract: must Err (or match llvm-mc rejection). Differential on accept/reject.
- Doc contract: gp_integer.rs:111 "infer size from operands"; default branch gp_integer.rs:121 `_ => 4` — limitation/asserted interaction fingerprint c4f81a22
- Seed: (none)
- Formal: ∀ imm, mem. llvm_mc rejects "mov $imm, mem" ⇒ encode("mov",[Imm,Mem]).is_err()
- Test file: src/backend/i686/assembler/encoder/encode_mov_infer_size_pbt.rs
- Status: failing
- Counterexample: mov $0, (%eax) → Ok([c7, 00, 00, 00, 00, 00]) (seed cc d01348f11493b4893fbc4fbc4fd75495baa5f95445f568547559ec9cc02c1242)
- Bug report: bug_reports/encode_mov_infer_size_ambiguous_imm_mem.md

```property
function: encode_mov_infer_size
oracle: differential
predicate:
  quantifier: forall
  vars: [imm, mem]
  domain: { imm: i32, mem: base_mem }
  body: "llvm_mc_rejects(asm) => sut_encode(\"mov\",[Imm,Mem]).is_err()"
generators:
  imm: { gen: int, min: -128, max: 127, type: i64 }
evidence: llvm-mc ambiguous-suffix error; gp_integer.rs:121
```

## encode_mov_infer_size_neg_mismatched_width
- Tier: 4
- Rationale: llvm-mc rejects unsuffixed `mov` between GP registers of different widths (`mov %eax, %ax`). SUT must not silently encode using first-register size. Differential accept/reject + byte agreement when both accept.
- Doc contract: gp_integer.rs:111 infer size — asserted fingerprint 7b2e4c91
- Seed: (none)
- Formal: ∀ src∈GP_w1, dst∈GP_w2, w1≠w2. llvm_mc rejects ⇒ encode("mov",[src,dst]).is_err()
- Test file: src/backend/i686/assembler/encoder/encode_mov_infer_size_pbt.rs
- Status: failing
- Counterexample: mov %ax, %al → Ok([66, 89, c0]) (first-reg size=2)
- Bug report: bug_reports/encode_mov_infer_size_mismatched_width.md

```property
function: encode_mov_infer_size
oracle: differential
predicate:
  quantifier: forall
  vars: [src, dst, w1, w2]
  domain: { w1 != w2, src: gp(w1), dst: gp(w2) }
  body: "llvm_mc_rejects(asm) => sut.is_err()"
generators:
  w1: { gen: oneof, choices: [1, 2, 4], type: u8 }
  w2: { gen: oneof, choices: [1, 2, 4], type: u8 }
evidence: llvm-mc "unknown use of instruction mnemonic without a size suffix"
```

## encode_mov_infer_size_diff_cr_seg
- Tier: 4
- Rationale: Strengthening round — unsuffixed `mov` through CR/Sreg must still reach encode_mov_cr/seg and match llvm-mc (size inference must not break specialized paths).
- Doc contract: gp_integer.rs:111 — asserted fingerprint 7b2e4c91
- Seed: encode_mov_cr_pbt.rs:321
- Formal: ∀ cr∈CR, gp∈GP32, sg∈Sreg. encode("mov", CR/GP or Sreg/GP pairs) = llvm_mc(...)
- Test file: src/backend/i686/assembler/encoder/encode_mov_infer_size_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_infer_size
oracle: differential
predicate:
  quantifier: forall
  vars: [kind, gp, cr_or_seg]
  domain: { kind: {cr_read,cr_write,seg_read,seg_write}, gp: GP32 }
  relation:
    op: eq
    lhs: "sut_encode(\"mov\", ops)"
    rhs: "llvm_mc_bytes(asm)"
generators:
  kind: { gen: int, min: 0, max: 3, type: u8 }
evidence: encode_mov routes CR/Sreg before size-sensitive GP path
```
