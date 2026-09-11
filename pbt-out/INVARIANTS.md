# Confirmed invariants (encode_add_sub)

- Immediate-form ADD/SUB/ADDS/SUBS with a valid imm12 or auto-shift (N<<12, N in 1..=0xFFF), including negative-imm alias, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Shifted-register form (LSL/LSR/ASR, Rd/Rn not SP, amount in range) matches llvm-mc.
- NEON vector ADD/SUB Vd.T, Vn.T, Vm.T for T in {8b,16b,4h,8h,2s,4s,2d} matches llvm-mc.
- Fewer than 3 operands always returns Err containing "requires 3 operands".
- encode_add_sub([Rd,Rn,Imm(-N)], is_sub, s) equals encode_add_sub([Rd,Rn,Imm(N)], !is_sub, s) for valid positive N.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- Immediate form register 31 is SP/WSP, never XZR/WZR (llvm-mc rejects `add Rd, XZR, #imm`).
- Known-answer: `add x0, x1, #42` encodes as 0x9100a820.

## Quirks

- llvm-mc may disassemble `add w0, wsp, #0` as `mov w0, wsp`; the encoding word still matches.
- proptest `prop_assert_eq!` format strings cannot use implicit captures (`{asm}`); use `{}` + args.
