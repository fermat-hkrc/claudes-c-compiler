# Confirmed invariants / environment quirks

## encode_in / encode_out (i686 system encoder)

- Intel IN/OUT fix data register to AL/AX/EAX and port to DX (or imm8).
- AT&T operand order: OUT is `outb %al, %dx` / `outb %al, $imm`; IN is `inb %dx, %al` / `inb $imm, %al`.
- AT&T parenthesized port `(%dx)` is accepted by llvm-mc as alias of `%dx` (same EC/ED or EE/EF bytes).
- Operand-size override: inw/outw = `[0x66] ++ inl/outl` for the same port shape.
- llvm-mc path used by campaigns: `/home/toan/tools/llvm15-official/bin/llvm-mc -triple=i686 -show-encoding`.
- proptest cases set explicitly to 1000 for standard tier.
- Known defect class (both encode_in and encode_out): (1) any Register,Register pair accepted; (2) `*val as u8` truncates OOR imm; (3) missing Memory port form on i686 (x86-64 sibling handles it).

## encode_invlpg (i686 system encoder)

- Opcode: 0F 01 /7 + ModR/M memory only (rejects register/imm/label).
- Non-segment base/disp/SIB/abs forms match llvm-mc `-triple=i686`.
- Metamorphic: same memory → invlpg Mod+RM/SIB/disp equals lidt; only ModRM.reg differs (7 vs 3).
- Known defect: does not call `emit_segment_prefix` before opcode (same class as encode_prefetch / encode_prefetch_0f0d). Witness: `invlpg %es:(%eax)` → SUT omits 0x26.
- Fix shape: `self.emit_segment_prefix(mem);` before `extend_from_slice(&[0x0F, 0x01])`.

## encode_verw (i686 system encoder)

- Opcode: 0F 00 /5 + ModR/M; Intel VERW is r/m16 (register form: ax/bx/cx/dx/sp/bp/si/di only).
- Non-segment base/disp/SIB/abs and r16 forms match llvm-mc `-triple=i686`.
- Known defects: (1) memory arm does not call `emit_segment_prefix` before opcode (same class as encode_invlpg/prefetch); (2) register arm accepts 32/8-bit names via `reg_num` aliasing (eax→same bytes as ax).
- Fix shapes: `self.emit_segment_prefix(mem);` before `extend_from_slice(&[0x0F, 0x00])`; gate register form with `reg_size(&reg.name) == 2`.

## Harness

- PBT files: `src/backend/i686/assembler/encoder/*_pbt.rs` + `#[cfg(test)] mod` in `encoder/mod.rs`.
- Build contract form: `cargo test --lib <filter> -- --test-threads=1`.
- llvm-mc path: `/home/toan/tools/llvm15-official/bin/llvm-mc -triple=i686 -show-encoding`.
- proptest cases set explicitly to 1000 for standard tier.

## encode_lsl (i686 system encoder)

- Opcode: 0F 03 /r (Load Segment Limit); AT&T `lsl src, dst` with dest in ModRM.reg.
- Same-width reg32/reg16 forms match llvm-mc `-triple=i686`.
- Operand-size override MUST follow destination width (Intel LSL r16/r32, r/m16), not source.
- Memory form must: (1) `emit_segment_prefix(mem)` before opcode; (2) emit 0x66 when dest is r16.
- Known defects: (1) memory arm skips segment prefix; (2) register arm keys is_16 off src; (3) memory arm never emits 0x66 for r16 dest.
- Fix shape: `is_16` on `dst.name`; memory arm `emit_segment_prefix` + optional 0x66 then `0F 03` + modrm_mem.

## encode_system_table (i686 system encoder)

- Opcode: 0F 01 /N with N={sgdt:0,sidt:1,lgdt:2,lidt:3}; optional `l` suffix stripped (`lgdtl`→`lgdt`).
- Memory forms (base/disp/SIB/abs, no segment) and Label absolute form match llvm-mc `-triple=i686`.
- Metamorphic: same memory → four mnemonics share mod+rm/SIB/disp; only ModRM.reg differs.
- Label form: mod=00 rm=101 + R_386_32 reloc + 4 zero bytes (llvm-mc `A,A,A,A`).
- Known defect: memory arm does not call `emit_segment_prefix` before opcode (same class as encode_invlpg/prefetch/verw/lsl). Witness: `sgdt %es:(%eax)` → SUT omits 0x26; `lgdt %fs:(%eax)` omits 0x64.
- Fix shape: `self.emit_segment_prefix(mem);` before `extend_from_slice(&[0x0F, 0x01])`.

## encode_lmsw (i686 system encoder)

- Opcode: 0F 01 /6 + ModR/M; Intel LMSW is r/m16 (register form: ax/bx/cx/dx/sp/bp/si/di only). No 0x66 prefix on register form (unlike SMSW).
- Non-segment base/disp/SIB/abs and r16 forms match llvm-mc `-triple=i686`.
- Metamorphic: same memory → lmsw Mod+RM/SIB/disp equals lidt; only ModRM.reg differs (6 vs 3).
- Known defects: (1) memory arm does not call `emit_segment_prefix` before opcode (same class as encode_invlpg/prefetch/verw/lsl/system_table). Witness: `lmsw %es:(%eax)` → SUT omits 0x26; `lmsw %fs:4(%esi)` omits 0x64. (2) register arm accepts 32/8-bit names via `reg_num` aliasing (eax→same bytes as ax).
- Fix shapes: `self.emit_segment_prefix(mem);` before `extend_from_slice(&[0x0F, 0x01])`; gate register form with `reg_size(&reg.name) == 2`.
