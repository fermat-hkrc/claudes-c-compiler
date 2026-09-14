# Properties: encode_shift

## encode_shift_diff_imm_reg
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent x86-64 assembler of the same GNU-style AT&T shift text). State machine rejected: single encoding call, no lifecycle. Round-trip rejected: no in-tree x86 shift decoder. Siblings encode_double_shift/encode_sse_shift/encode_avx_shift/encode_bmi2_shift fail the same-job gate. SUT-boundary: internal-helper of the GNU-style x86-64 assembler; mapping [Imm(count), Reg(dst)] + mnemonic/shift_op <-> mnemonic $count, %dst.
- Seed: src/backend/x86/codegen/emit.rs:151 emits shll/shlq
- Formal: ∀ mnemonic ∈ {shl,shr,sar,rol,ror,rcl,rcr}×{b,w,l,q}, dst a GP register of that size, count ∈ 0..255. encode_shift([Imm(count), Reg(dst)], mnemonic, shift_op).bytes = llvm-mc(-triple=x86_64, asm)
- Test file: src/backend/x86/assembler/encoder/gp_integer.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_shift
oracle: differential
predicate:
  quantifier: forall
  vars: [kind, size_suf, dst, count]
  domain: { kind: shift_kind, size_suf: bwlq, dst: gp_of_size, count: 0..255 }
  relation:
    op: eq
    lhs: encode_shift([Imm(count), Reg(dst)], mnemonic, shift_op).bytes
    rhs: llvm_mc_bytes(asm)
generators:
  kind: { gen: oneof, items: [shl, shr, sar, rol, ror, rcl, rcr] }
  size_suf: { gen: oneof, items: [b, w, l, q] }
  dst: { gen: string }
  count: { gen: int, min: 0, max: 255, type: i64 }
evidence: "README.md:567 Shifts/Rotates; encoder/mod.rs:196-200,670-671 dispatch; Intel SDM Group 2 C0/C1/D0/D1; GAS AT&T count, dest"
```

## encode_shift_diff_cl_reg
- Tier: 2
- Rationale: Intel SDM documents the CL count form (D2/D3 /r). Same differential oracle and mapping as imm form with ops[0]=Reg(cl).
- Seed: registers.rs:176-178 shl %cl, %edx should become shll
- Formal: ∀ mnemonic matching dest size, dst GP of that size. encode_shift([Reg(cl), Reg(dst)], mnemonic, shift_op).bytes = llvm-mc(mnemonic %cl, %dst)
- Test file: src/backend/x86/assembler/encoder/gp_integer.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_shift
oracle: differential
predicate:
  quantifier: forall
  vars: [kind, size_suf, dst]
  domain: { kind: shift_kind, size_suf: bwlq, dst: gp_of_size }
  relation:
    op: eq
    lhs: encode_shift([Reg(cl), Reg(dst)], mnemonic, shift_op).bytes
    rhs: llvm_mc_bytes(asm)
generators:
  kind: { gen: oneof, items: [shl, shr, sar, rol, ror, rcl, rcr] }
  size_suf: { gen: oneof, items: [b, w, l, q] }
  dst: { gen: string }
evidence: "Intel SDM D2/D3 /r shift r/m, CL; registers.rs:176-178 shift size from dest not cl"
```

## encode_shift_diff_one_operand
- Tier: 2
- Rationale: Body comment gp_integer.rs:759 and GAS: omitted count is assumed 1. 1-operand form must match llvm-mc mnemonic %dst (D0/D1).
- Seed: gp_integer.rs:759 Handle 1-operand form: shift by 1 implicitly
- Formal: ∀ mnemonic matching dest size, dst GP of that size. encode_shift([Reg(dst)], mnemonic, shift_op).bytes = llvm-mc(mnemonic %dst)
- Test file: src/backend/x86/assembler/encoder/gp_integer.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_shift
oracle: differential
predicate:
  quantifier: forall
  vars: [kind, size_suf, dst]
  domain: { kind: shift_kind, size_suf: bwlq, dst: gp_of_size }
  relation:
    op: eq
    lhs: encode_shift([Reg(dst)], mnemonic, shift_op).bytes
    rhs: llvm_mc_bytes(asm)
generators:
  kind: { gen: oneof, items: [shl, shr, sar, rol, ror, rcl, rcr] }
  size_suf: { gen: oneof, items: [b, w, l, q] }
  dst: { gen: string }
evidence: "gp_integer.rs:759 implicit shift by 1; GAS omitted count is 1; Intel SDM SAL r/m = SAL r/m, 1"
```

## encode_shift_meta_one_eq_imm1
- Tier: 4
- Rationale: GAS/Intel: omitted count equals count 1. Metamorphic: 1-operand encoding equals 2-operand Imm(1) encoding. Independent of llvm-mc.
- Seed: gp_integer.rs:759
- Formal: ∀ mnemonic, dst GP of matching size. encode_shift([Reg(dst)], m, op).bytes = encode_shift([Imm(1), Reg(dst)], m, op).bytes
- Test file: src/backend/x86/assembler/encoder/gp_integer.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_shift
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [kind, size_suf, dst]
  domain: { kind: shift_kind, size_suf: bwlq, dst: gp_of_size }
  relation:
    op: eq
    lhs: encode_shift([Reg(dst)], mnemonic, shift_op).bytes
    rhs: encode_shift([Imm(1), Reg(dst)], mnemonic, shift_op).bytes
generators:
  kind: { gen: oneof, items: [shl, shr, sar, rol, ror, rcl, rcr] }
  size_suf: { gen: oneof, items: [b, w, l, q] }
  dst: { gen: string }
evidence: "GAS omitted count is 1; Intel SDM SAL r/m encoding equals SAL r/m, 1 (D0/D1)"
```

## encode_shift_meta_shift_op_digit
- Tier: 4
- Rationale: Intel SDM Group 2: the /digit field is the only difference between ROL/ROR/RCL/RCR/SHL/SHR/SAR at equal size, dest, and count. Metamorphic: encodings differ only in ModR/M bits [5:3].
- Seed: encoder/mod.rs:196-200 shift_op 4/5/7/0/1
- Formal: ∀ dst 64-bit GP, count ∈ 0..255, op_a ≠ op_b ∈ {0,1,2,3,4,5,7}. encode_shift(ops, mnemonic, op_a) and encode_shift(ops, mnemonic, op_b) differ only in ModR/M /digit
- Test file: src/backend/x86/assembler/encoder/gp_integer.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_shift
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [dst, count, op_a, op_b]
  domain: { dst: gp64, count: 0..255, op_a: group2_digit, op_b: group2_digit }
  relation:
    op: holds
    expr: encodings_differ_only_in_modrm_digit(op_a, op_b)
generators:
  dst: { gen: string }
  count: { gen: int, min: 0, max: 255, type: i64 }
  op_a: { gen: int, min: 0, max: 7, type: u8 }
  op_b: { gen: int, min: 0, max: 7, type: u8 }
evidence: "Intel SDM Group 2 /digit ROL=0 ROR=1 RCL=2 RCR=3 SHL=4 SHR=5 SAR=7"
```

## encode_shift_diff_mem
- Tier: 2
- Rationale: Same GAS contract for r/m dest. Memory forms (%base) including rsp/r12 (SIB) and rbp/r13 (disp8) plus optional fs/gs must match llvm-mc. emit_segment_prefix TODO at core.rs:47-48 says other instruction families that accept memory should emit segment overrides.
- Seed: README.md:567 r/m forms; core.rs:47-48 segment-prefix TODO
- Formal: ∀ mnemonic, base ∈ GP64, form ∈ {imm, cl, 1-op}, seg ∈ {none, fs, gs}. encode_shift(mem-ops).bytes = llvm-mc(asm)
- Test file: src/backend/x86/assembler/encoder/gp_integer.rs
- Status: failing
- Counterexample: shlb $0, %fs:(%rax) → SUT [0xc0,0x20,0x00] vs llvm-mc [0x64,0xc0,0x20,0x00]
- Bug report: pbt-out/bug_reports/encode_shift_missing_segment_prefix.md

```property
function: encode_shift
oracle: differential
predicate:
  quantifier: forall
  vars: [kind, size_suf, base, form, count, seg]
  domain: { kind: shift_kind, size_suf: bwlq, base: gp64, form: imm_cl_one, count: 0..255, seg: none_fs_gs }
  relation:
    op: eq
    lhs: encode_shift(mem_ops, mnemonic, shift_op).bytes
    rhs: llvm_mc_bytes(asm)
generators:
  kind: { gen: oneof, items: [shl, shr, sar, rol, ror, rcl, rcr] }
  size_suf: { gen: oneof, items: [b, w, l, q] }
  base: { gen: string }
  form: { gen: oneof, items: [imm, cl, one] }
  count: { gen: int, min: 0, max: 255, type: i64 }
  seg: { gen: oneof, items: [none, fs, gs] }
evidence: "README.md:567 r/m dest; Intel SDM r/m; core.rs:47-48 segment override must precede opcode"
```

## encode_shift_neg_arity_non_cl
- Tier: 4
- Rationale: Body requires 1 or 2 operands (gp_integer.rs:759,781). Intel SDM/GAS require the register count to be CL, not another GP. llvm-mc rejects extra operands and non-CL count. Negative/error contract.
- Seed: gp_integer.rs:781 requires 2 operands; llvm-mc rejects shlq %rcx, %rax
- Formal: ∀ mnemonic, ops with len not in {1,2} or (len=2 and ops[0] is Reg not equal to cl and ops[1] is matching GP). encode_shift(ops, mnemonic, shift_op) = Err
- Test file: src/backend/x86/assembler/encoder/gp_integer.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_shift
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, size_suf, dst, arity, count_reg]
  domain: { kind: shift_kind, size_suf: bwlq, dst: gp_of_size, arity: 0..4, count_reg: gp_except_cl }
  relation:
    op: holds
    expr: encode_shift(ops, mnemonic, shift_op).is_err()
generators:
  kind: { gen: oneof, items: [shl, shr, sar, rol, ror, rcl, rcr] }
  size_suf: { gen: oneof, items: [b, w, l, q] }
  dst: { gen: string }
  arity: { gen: int, min: 0, max: 4, type: usize }
  count_reg: { gen: string }
expected_error: String
evidence: "gp_integer.rs:781 arity; Intel SDM count is 1 or CL or imm8; llvm-mc rejects non-CL register count"
```

## encode_shift_neg_mixed_size_and_non_gp
- Tier: 4
- Rationale: llvm-mc and Intel SDM reject size-mismatched dest (shlq %eax, shlb %rax) and non-GP dest (%xmm0, %st). README claims AT&T/GAS compatibility. parse/encode path is caller-reachable: parser stores the register name as-is. Negative/error contract: encode_shift must Err.
- Seed: llvm-mc rejects shlq $1, %eax and shlq $1, %xmm0; README.md:5-14 GAS-compatible subset
- Formal: ∀ mnemonic of size S, dest a register whose width ≠ S or dest in {xmm,ymm,mm,st,seg}. encode_shift([Imm(1), Reg(dest)], mnemonic, shift_op) = Err
- Test file: src/backend/x86/assembler/encoder/gp_integer.rs
- Status: failing
- Counterexample: shlw $1, %al → Ok([0x66,0xd1,0xe0]) (encodes as shlw %ax); shlq $1, %xmm0 → Ok([0x48,0xd1,0xe0]) (encodes as shlq %rax)
- Bug report: pbt-out/bug_reports/encode_shift_accepts_mismatched_and_non_gp_dest.md

```property
function: encode_shift
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, size_suf, dest]
  domain: { kind: shift_kind, size_suf: bwlq, dest: mismatched_or_non_gp }
  relation:
    op: holds
    expr: encode_shift([Imm(1), Reg(dest)], mnemonic, shift_op).is_err()
generators:
  kind: { gen: oneof, items: [shl, shr, sar, rol, ror, rcl, rcr] }
  size_suf: { gen: oneof, items: [b, w, l, q] }
  dest: { gen: string }
expected_error: String
evidence: "llvm-mc rejects size mismatch and xmm dest; Intel SDM Group 2 r/m is GP or memory; README.md:5-14 AT&T/GAS"
```

## encode_shift_diff_mem_no_seg
- Tier: 2
- Rationale: Same differential oracle as encode_shift_diff_mem restricted to no segment override, so SIB (rsp/r12) and mandatory disp8 (rbp/r13) memory encodings are still checked independently of the FS/GS prefix bug.
- Seed: README.md:567 r/m dest
- Formal: ∀ mnemonic, base ∈ GP64, form ∈ {imm, cl, 1-op}. encode_shift(mem-ops without segment).bytes = llvm-mc(asm)
- Test file: src/backend/x86/assembler/encoder/gp_integer.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_shift
oracle: differential
predicate:
  quantifier: forall
  vars: [kind, size_suf, base, form, count]
  domain: { kind: shift_kind, size_suf: bwlq, base: gp64, form: imm_cl_one, count: 0..255 }
  relation:
    op: eq
    lhs: encode_shift(mem_ops, mnemonic, shift_op).bytes
    rhs: llvm_mc_bytes(asm)
generators:
  kind: { gen: oneof, items: [shl, shr, sar, rol, ror, rcl, rcr] }
  size_suf: { gen: oneof, items: [b, w, l, q] }
  base: { gen: string }
  form: { gen: oneof, items: [imm, cl, one] }
  count: { gen: int, min: 0, max: 255, type: i64 }
evidence: "README.md:567 r/m dest; Intel SDM r/m; rsp/r12 need SIB; rbp/r13 need disp8"
```

## encode_shift_neg_imm_overflow
- Tier: 4
- Rationale: Intel SDM shift immediate is imm8; llvm-mc rejects $256. encode_shift truncates via `count as u8`. Negative/error contract: counts outside 0..255 (and outside the signed wrap llvm-mc accepts) must Err.
- Seed: llvm-mc rejects shlq $256, %rax
- Formal: ∀ mnemonic matching dest size, count ∈ 256..1024. encode_shift([Imm(count), Reg(dst)], mnemonic, shift_op) = Err
- Test file: src/backend/x86/assembler/encoder/gp_integer.rs
- Status: failing
- Counterexample: shlb $256, %al → Ok (truncated to 0); shlq $256, %rax → Ok([0x48,0xc1,0xe0,0x00])
- Bug report: pbt-out/bug_reports/encode_shift_truncates_imm8_count.md

```property
function: encode_shift
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, size_suf, dst, count]
  domain: { kind: shift_kind, size_suf: bwlq, dst: gp_of_size, count: 256..1024 }
  relation:
    op: holds
    expr: encode_shift([Imm(count), Reg(dst)], mnemonic, shift_op).is_err()
generators:
  kind: { gen: oneof, items: [shl, shr, sar, rol, ror, rcl, rcr] }
  size_suf: { gen: oneof, items: [b, w, l, q] }
  dst: { gen: string }
  count: { gen: int, min: 256, max: 1024, type: i64 }
expected_error: String
evidence: "Intel SDM Group 2 imm8; llvm-mc rejects $256"
```

## encode_shift_neg_one_operand_non_rm
- Tier: 4
- Rationale: 1-operand form only accepts Register or Memory (gp_integer.rs:761-776). Other kinds must Err with unsupported operand. Coverage-sweep of the documented `_` error arm.
- Seed: gp_integer.rs:776 unsupported operand
- Formal: ∀ mnemonic, ops = [Imm] or [Label] or [Indirect]. encode_shift(ops, mnemonic, shift_op) = Err
- Test file: src/backend/x86/assembler/encoder/gp_integer.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_shift
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, size_suf, which]
  domain: { kind: shift_kind, size_suf: bwlq, which: 0..2 }
  relation:
    op: holds
    expr: encode_shift([non_rm], mnemonic, shift_op).is_err()
generators:
  kind: { gen: oneof, items: [shl, shr, sar, rol, ror, rcl, rcr] }
  size_suf: { gen: oneof, items: [b, w, l, q] }
  which: { gen: int, min: 0, max: 2, type: u8 }
expected_error: String
evidence: "gp_integer.rs:776 unsupported operand for 1-operand non-r/m"
```

## encode_shift_rip_reloc_addend
- Tier: 4
- Rationale: encode_shift calls adjust_rip_reloc_addend(rc, 1) after encoding a memory dest with a trailing imm8 (count != 1). Documented at core.rs:247-253: trailing immediate bytes need A = -(4 + trailing_bytes). Coverage-sweep of the RIP-relative memory + imm path.
- Seed: gp_integer.rs:814 adjust_rip_reloc_addend(rc, 1); core.rs:247-253
- Formal: ∀ mnemonic, count ∈ 2..255. encode_shift([Imm(count), Mem(foo(%rip))], mnemonic, shift_op) produces one R_X86_64_PC32 reloc of symbol foo with addend -5
- Test file: src/backend/x86/assembler/encoder/gp_integer.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_shift
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [kind, size_suf, count]
  domain: { kind: shift_kind, size_suf: bwlq, count: 2..255 }
  relation:
    op: holds
    expr: reloc.addend == -5 && reloc.reloc_type == R_X86_64_PC32 && reloc.symbol == foo
generators:
  kind: { gen: oneof, items: [shl, shr, sar, rol, ror, rcl, rcr] }
  size_suf: { gen: oneof, items: [b, w, l, q] }
  count: { gen: int, min: 2, max: 255, type: i64 }
evidence: "gp_integer.rs:814; core.rs:247-253 trailing imm8 adjusts RIP addend by -1"
```
