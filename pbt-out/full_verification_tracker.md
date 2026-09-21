# Full-Tracker Verification — all 509 open issues, sorted (2026-09-15)

Method: Contract-Based Differential Validation — 503 ARM issues via assembler-CLI pipeline
(ccc-arm vs aarch64-linux-gnu-gcc 13.3 vs clang/llvm-mc, objdump encoding comparison, -march ladder;
x86 issues via host tools); 6 C-level/encoding issues verified individually (see manual_tests.md § #2-#510).

| # | Input | ccc | gcc | clang | Verdict |
|---|---|---|---|---|---|
| 2 | (C-level x86-64) cast_float_to_target(F128 subnormal) panic | see manual_tests.md | see manual_tests.md | — | real — panic + silent miscompile vs host gcc + qemu |
| 3 | (C-level x86-64) cast_float_to_target U8/U16 signed storage | see manual_tests.md | see manual_tests.md | — | real — IR-contract violation (latent at C level) |
| 4 | (C-level i686) classify_cast_with_f128 Ptr signed | see manual_tests.md | see manual_tests.md | — | real — unit+asm+qemu; gcc 0x0b2047e0 vs ccc 0x0b0007e0 |
| 5 | `adc w0, w0, w0, lsl #0` | ACCEPT → 0x1a000000 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 6 | `adc d0, x1, x2` | ACCEPT → 0x1a020020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 7 | `adc w0, w0, x0` | ACCEPT → 0x1a000000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 8 | `adc wsp, w0, w0` | ACCEPT → 0x1a00001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 9 | `adds wsp, w0, #0` | ACCEPT → 0x3100001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 10 | `add d0, x1, x2` | ACCEPT → 0x5ee28420 | ERR: expected a scalar SIMD or floating-point register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 11 | `add w0, w1, #4097, lsl #12` | ACCEPT → 0x11400420 | ERR: immediate out of range | ERR: expected compatible register, symbol or integer in range [0, 4095] | silent-accept |
| 12 | `add w0, w0, #0, lsr #0` | ACCEPT → 0x11000000 | ERR: only 'LSL' shift is permitted at operand 3 | ERR: only 'lsl #+N' valid after immediate | silent-accept |
| 13 | `add x0, w0, w0` | ACCEPT → 0x8b000000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 14 | `add w0, w1, w2, ror #0` | ACCEPT → 0x0b020020 | ERR: 'ROR' operator not allowed at operand 3 | ERR: expected 'sxtx' 'uxtx' or 'lsl' with optional integer in range [0, 4] | silent-accept |
| 15 | `add w0, wsp, w0, lsl #1` | ACCEPT → 0x0b0007e0 | ACCEPT → 0x0b2047e0 | ACCEPT → 0x0b2047e0 | differential |
| 16 | `adr d0, #-1048576` | ACCEPT → 0x10800000 | ERR: expected an integer register or SVE vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 17 | `adr x0, #-1048577` | ACCEPT → 0x707fffe0 | ERR: immediate out of range at operand 2 | ERR: expected label or encodable integer pc offset | silent-accept |
| 18 | `adr x0, :lo12:foo` | ACCEPT → 0x10000000 | ERR: this relocation modifier is not allowed on this instruction at operand 2 | ERR: unexpected adr label | silent-accept |
| 19 | `adr sp, #-1048576` | ACCEPT → 0x1080001f | ERR: expected an integer register or SVE vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 20 | `adr w0, #-1048576` | ACCEPT → 0x10800000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 21 | `bic d0, x1, x2` | ACCEPT → 0x0a220020 | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 22 | `bic wzr, w0, #1` | ACCEPT → 0x121f781f | ERR: integer register expected in the extended/shifted operand register at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 23 | `bic v0.8h, v1.8h, v2.8h` | ACCEPT → 0x0e621c20 | ERR: operand mismatch | ERR: immediate must be an integer in range [0, 255]. | silent-accept |
| 24 | `bic w0, w0, x0` | ACCEPT → 0x0a200000 | ERR: operand mismatch | ERR: expected compatible register or logical immediate | silent-accept |
| 25 | `bic w0, w0, w0, lsl #32` | ACCEPT → 0x0a208000 | ERR: shift amount out of range 0 to 31 at operand 3 | ERR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31] | silent-accept |
| 26 | `bic wsp, w0, w0` | ACCEPT → 0x0a20001f | ERR: unexpected register in the immediate operand at operand 3 | ERR: expected compatible register or logical immediate | silent-accept |
| 27 | `bic w0, w0, w0, lslx #0` | ACCEPT → 0x0a200000 | ERR: unexpected register in the immediate operand at operand 3 | ERR: unexpected token in argument list | silent-accept |
| 28 | `bics w0, w0, w0, w0` | ACCEPT → 0x6a200000 | ERR: shift operator expected at operand 3 | ERR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31] | silent-accept |
| 29 | `bics d0, x1, x2` | ACCEPT → 0x6a220020 | ERR: expected an integer or predicate register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 30 | `bics w0, w0, #0xaaaaaaaa` | ERR: expected register at operand 2, got Some(Imm(2863311530)) | ERR: integer register expected in the extended/shifted operand register at operand 3 | ACCEPT → 0x7200f000 | not-a-defect (gas parity): llvm-mc-only alias — enhancement request |
| 31 | `bics w0, w0, x0` | ACCEPT → 0x6a200000 | ERR: operand mismatch | ERR: expected compatible register or logical immediate | silent-accept |
| 32 | `bics w0, w0, w0, lsl #32` | ACCEPT → 0x6a208000 | ERR: shift amount out of range 0 to 31 at operand 3 | ERR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31] | silent-accept |
| 33 | `bics wsp, w0, w0` | ACCEPT → 0x6a20001f | ERR: expected an integer or predicate register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 34 | `bics w0, w0, w0, lslx #0` | ACCEPT → 0x6a200000 | ERR: shift operator expected at operand 3 | ERR: unexpected token in argument list | silent-accept |
| 35 | `bl labl0, x0` | ACCEPT → 0x94000000 | ERR: unexpected characters following instruction at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 36 | `bl #-134217728` | ERR: expected symbol at operand 0, got Some(Imm(-134217728)) | ACCEPT → 0x96000000 | ACCEPT → 0x96000000 | reverse |
| 37 | `bl :lo12:foo` | ERR: unsupported instruction: foo | ERR: unknown mnemonic `foo' | ERR: unrecognized instruction mnemonic | real — parser-masked at CLI (unit-proven: encoder accepts, sample #37) |
| 38 | `addhn v0.8b, v0.8h, v0.8h, v0.8h` | ACCEPT → 0x0e204000 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 39 | `addhn x0, v0.8h, v0.8h` | ACCEPT → 0x0e204000 | ERR: expected an Advanced SIMD vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 40 | `addhn2 v0.8b, v0.8h, v0.8h` | ACCEPT → 0x4e204000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 41 | `addhn v0.4h, v0.4s, v0.8h` | ACCEPT → 0x0e604000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 42 | `blr x0, x1` | ACCEPT → 0xd63f0000 | ERR: unexpected characters following instruction at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 43 | `blr d0` | ACCEPT → 0xd63f0000 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 44 | `blr sp` | ACCEPT → 0xd63f03e0 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 45 | `blr w0` | ACCEPT → 0xd63f0000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 46 | `br x0, x1` | ACCEPT → 0xd61f0000 | ERR: unexpected characters following instruction at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 47 | `br d0` | ACCEPT → 0xd61f0000 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 48 | `br sp` | ACCEPT → 0xd61f03e0 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 49 | `br w0` | ACCEPT → 0xd61f0000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 50 | `b labl0, x0` | ACCEPT → 0x14000000 | ERR: unexpected characters following instruction at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 51 | `b #0` | ERR: expected symbol at operand 0, got Some(Imm(0)) | ACCEPT → 0x14000000 | ACCEPT → 0x14000000 | reverse |
| 52 | `b :lo12:foo` | ERR: unsupported instruction: foo | ERR: unknown mnemonic `foo' | ERR: unrecognized instruction mnemonic | real — parser-masked at CLI (encoder accepts per issue; parser rejects :lo12: text) |
| 53 | `cbz x0, L, x1` | ACCEPT → 0xb4000020 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 54 | `cbz d0, L` | ACCEPT → 0x34000020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 55 | `cbz x0, #-1048576` | ERR: expected symbol at operand 1, got Some(Imm(-1048576)) | ACCEPT → 0xb4800000 | ACCEPT → 0xb4800000 | reverse |
| 56 | `cbz sp, L` | ACCEPT → 0xb400003f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 57 | `ccmn x0, #0, #0, eq, x1` | ACCEPT → 0xba400800 | ERR: unexpected characters following instruction at operand 4 | ERR: invalid operand for instruction | silent-accept |
| 58 | `ccmp d0, #0, #0, eq` | ACCEPT → 0x7a400800 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 59 | `ccmn x0, #-1, #0, eq` | ACCEPT → 0xba5f0800 | ERR: immediate value out of range 0 to 31 at operand 2 | ERR: immediate must be an integer in range [0, 31]. | silent-accept |
| 60 | `ccmn w0, x0, #0, eq` | ACCEPT → 0x3a400000 | ERR: operand mismatch | ERR: immediate must be an integer in range [0, 31]. | silent-accept |
| 61 | `ccmn sp, #0, #0, eq` | ACCEPT → 0xba400be0 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 62 | `cinc x0, x0, al` | ACCEPT → 0x9a80f400 | ERR: operand 3 must be one of the standard conditions, excluding AL and NV. | ERR: condition codes AL and NV are invalid for this instruction | silent-accept |
| 63 | `cinc x0, x0, eq, x2` | ACCEPT → 0x9a801400 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 64 | `cinc d0, d0, eq` | ACCEPT → 0x1a801400 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 65 | `cinc x0, w0, eq` | ACCEPT → 0x9a801400 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 66 | `cinc sp, x0, eq` | ACCEPT → 0x9a80141f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 67 | `cinv x0, x0, al` | ACCEPT → 0xda80f000 | ERR: operand 3 must be one of the standard conditions, excluding AL and NV. | ERR: condition codes AL and NV are invalid for this instruction | silent-accept |
| 68 | `cinv x0, x0, eq, x2` | ACCEPT → 0xda801000 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 69 | `cinv d0, d0, eq` | ACCEPT → 0x5a801000 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 70 | `cinv x0, w0, eq` | ACCEPT → 0xda801000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 71 | `cinv sp, x0, eq` | ACCEPT → 0xda80101f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 72 | `cmn x0, x0, x2` | ACCEPT → 0xab00001f | ERR: unexpected register in the immediate operand at operand 2 | ERR: expected 'sxtx' 'uxtx' or 'lsl' with optional integer in range [0, 4] | silent-accept |
| 73 | `cmn d0, #0` | ACCEPT → 0xb100001f | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 74 | `cmn x0, #i64::MIN` | WARN: failed to resolve deferred instruction 'cmn': unsupported add/sub operands: [Reg("xzr"), R | ERR: unexpected characters following instruction at operand 2 | ERR: unexpected token in argument list | real — warn-accepts what references reject (deferred-imm warning; latent encoder panic per issue) |
| 75 | `cmn x0, w0` | ACCEPT → 0xab00001f | ERR: missing extend operator at operand 2 | ERR: too few operands for instruction | silent-accept |
| 76 | `cmn x0, sp` | ACCEPT → 0xab1f001f | ERR: unexpected register in the immediate operand at operand 2 | ERR: expected compatible register, symbol or integer in range [0, 4095] | silent-accept |
| 77 | `cmn xzr, #0` | ACCEPT → 0xb10003ff | ERR: integer register expected in the extended/shifted operand register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 78 | `cmp x0, x0, x2` | ACCEPT → 0xeb00001f | ERR: unexpected register in the immediate operand at operand 2 | ERR: expected 'sxtx' 'uxtx' or 'lsl' with optional integer in range [0, 4] | silent-accept |
| 79 | `cmp d0, #0` | ACCEPT → 0xf100001f | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 80 | `cmp x0, #i64::MIN` | WARN: failed to resolve deferred instruction 'cmp': unsupported add/sub operands: [Reg("xzr"), R | ERR: unexpected characters following instruction at operand 2 | ERR: unexpected token in argument list | real — warn-accepts what references reject (deferred-imm warning; latent encoder panic per issue) |
| 81 | `cmp x0, w0` | ACCEPT → 0xeb00001f | ERR: missing extend operator at operand 2 | ERR: too few operands for instruction | silent-accept |
| 82 | `cmp x0, sp` | ACCEPT → 0xeb1f001f | ERR: unexpected register in the immediate operand at operand 2 | ERR: expected compatible register, symbol or integer in range [0, 4095] | silent-accept |
| 83 | `cmp xzr, #0` | ACCEPT → 0xf10003ff | ERR: integer register expected in the extended/shifted operand register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 84 | `cneg x0, x0, al` | ACCEPT → 0xda80f400 | ERR: operand 3 must be one of the standard conditions, excluding AL and NV. | ERR: condition codes AL and NV are invalid for this instruction | silent-accept |
| 85 | `cneg x0, x0, eq, x2` | ACCEPT → 0xda801400 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 86 | `cneg d0, d0, eq` | ACCEPT → 0x5a801400 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 87 | `cneg x0, w0, eq` | ACCEPT → 0xda801400 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 88 | `cneg sp, x0, eq` | ACCEPT → 0xda80141f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 89 | `csel x0, x1, x2, eq, x3` | ACCEPT → 0x9a820020 | ERR: unexpected characters following instruction at operand 4 | ERR: invalid operand for instruction | silent-accept |
| 90 | `csel d0, d1, d2, eq` | ACCEPT → 0x1a820020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 91 | `csel x0, w1, x2, eq` | ACCEPT → 0x9a820020 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 92 | `csel sp, x0, x1, eq` | ACCEPT → 0x9a81001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 93 | `cset x0, al` | ACCEPT → 0x9a9ff7e0 | ERR: operand 2 must be one of the standard conditions, excluding AL and NV. | ERR: condition codes AL and NV are invalid for this instruction | silent-accept |
| 94 | `cset x0, eq, x2` | ACCEPT → 0x9a9f17e0 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 95 | `cset d0, eq` | ACCEPT → 0x1a9f17e0 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 96 | `cset sp, eq` | ACCEPT → 0x9a9f17ff | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 97 | `csetm x0, al` | ACCEPT → 0xda9ff3e0 | ERR: operand 2 must be one of the standard conditions, excluding AL and NV. | ERR: condition codes AL and NV are invalid for this instruction | silent-accept |
| 98 | `csetm x0, eq, x2` | ACCEPT → 0xda9f13e0 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 99 | `csetm d0, eq` | ACCEPT → 0x5a9f13e0 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 100 | `csetm sp, eq` | ACCEPT → 0xda9f13ff | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 101 | `csinc x0, x0, x0, eq, x3` | ACCEPT → 0x9a800400 | ERR: unexpected characters following instruction at operand 4 | ERR: invalid operand for instruction | silent-accept |
| 102 | `csinc d0, d1, d2, eq` | ACCEPT → 0x1a820420 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 103 | `csinc x0, w1, x2, eq` | ACCEPT → 0x9a820420 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 104 | `csinc sp, x0, x0, eq` | ACCEPT → 0x9a80041f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 105 | `csinv x0, x0, x0, eq, x3` | ACCEPT → 0xda800000 | ERR: unexpected characters following instruction at operand 4 | ERR: invalid operand for instruction | silent-accept |
| 106 | `csinv d0, d1, d2, eq` | ACCEPT → 0x5a820020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 107 | `csinv x0, w1, x2, eq` | ACCEPT → 0xda820020 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 108 | `csinv sp, x0, x0, eq` | ACCEPT → 0xda80001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 109 | `csneg x0, x0, x0, eq, x3` | ACCEPT → 0xda800400 | ERR: unexpected characters following instruction at operand 4 | ERR: invalid operand for instruction | silent-accept |
| 110 | `csneg d0, d1, d2, eq` | ACCEPT → 0x5a820420 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 111 | `csneg x0, w1, x2, eq` | ACCEPT → 0xda820420 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 112 | `csneg sp, x0, x0, eq` | ACCEPT → 0xda80041f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 113 | `sdiv w0, w0, w0, x0` | ACCEPT → 0x1ac00c00 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 114 | `sdiv d0, x1, x2` | ACCEPT → 0x1ac20c20 | ERR: expected an integer register or SVE vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 115 | `sdiv w0, w0, x0` | ACCEPT → 0x1ac00c00 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 116 | `sdiv wsp, w0, w0` | ACCEPT → 0x1ac00c1f | ERR: expected an integer register or SVE vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 117 | `eon w0, w0, w0, w0` | ACCEPT → 0x4a200000 | ERR: shift operator expected at operand 3 | ERR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31] | silent-accept |
| 118 | `eon d0, x1, x2` | ACCEPT → 0x4a220020 | ERR: expected an integer register or SVE vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 119 | `eon w0, w0, #0xaaaaaaaa` | ERR: expected register at operand 2, got Some(Imm(2863311530)) | ERR: integer register expected in the extended/shifted operand register at operand 3 | ACCEPT → 0x5200f000 | not-a-defect (gas parity): llvm-mc-only alias — enhancement request |
| 120 | `eon w0, w0, x0` | ACCEPT → 0x4a200000 | ERR: operand mismatch | ERR: expected compatible register or logical immediate | silent-accept |
| 121 | `eon w0, w0, w0, lsl #32` | ACCEPT → 0x4a208000 | ERR: shift amount out of range 0 to 31 at operand 3 | ERR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31] | silent-accept |
| 122 | `eon wsp, w0, w0` | ACCEPT → 0x4a20001f | ERR: expected an integer register or SVE vector register at operand 1 | ERR: expected compatible register or logical immediate | silent-accept |
| 123 | `eon w0, w0, w0, lslx #0` | ACCEPT → 0x4a200000 | ERR: shift operator expected at operand 3 | ERR: unexpected token in argument list | silent-accept |
| 124 | `stlr w0, [x0], x2` | ACCEPT → 0x889ffc00 | ERR: unexpected address writeback at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 125 | `stlr sp, [x0]` | ACCEPT → 0xc89ffc1f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 126 | `stlr w0, [w0]` | ACCEPT → 0x889ffc00 | ERR: expected a 64-bit base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 127 | `saddlv b0, v0.8b` | ACCEPT → 0x0e303800 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 128 | `saddlv h0, v0.8b, h0` | ACCEPT → 0x0e303800 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 129 | `saddlv h0, v0.2s` | ACCEPT → 0x0eb03800 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 130 | `fcmeq v0.4s, v0.2s, #0.0` | WARN: failed to resolve deferred instruction 'fcmeq': expected NEON register at operand 2, got S | ERR: operand mismatch | ERR: invalid operand for instruction | real — warn-accepts what references reject (deferred-imm warning; latent encoder panic per issue) |
| 131 | `fcmeq v0.2s, v0.2s, #0.0, v0.2s` | WARN: failed to resolve deferred instruction 'fcmeq': expected NEON register at operand 2, got S | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | real — warn-accepts what references reject (deferred-imm warning; latent encoder panic per issue) |
| 132 | `fcmeq x0.2s, v0.2s, #0.0` | WARN: failed to resolve deferred instruction 'fcmeq': expected NEON register at operand 2, got S | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | real — warn-accepts what references reject (deferred-imm warning; latent encoder panic per issue) |
| 133 | `sli v0.8b, v0.16b, #0` | ACCEPT → 0x2f085400 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 134 | `sli v0.8b, v0.8b, #0, v0.8b` | ACCEPT → 0x2f085400 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 135 | `sli x0.8b, v0.8b, #0` | ACCEPT → 0x2f085400 | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 136 | `sli v0.8b, v0.8b, #-1` | PANIC | ERR: immediate value out of range 0 to 7 at operand 3 | ERR: immediate must be an integer in range [0, 7]. | panic-on-invalid |
| 137 | `stur w0, [x0, #-256], x2` | ACCEPT → 0xb8100000 | ERR: cannot combine pre- and post-indexing at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 138 | `stur w0, [x0, #-257]` | ACCEPT → 0xb80ff000 | ERR: immediate offset out of range -256 to 255 at operand 2 | ERR: index must be an integer in range [-256, 255]. | silent-accept |
| 139 | `stur lr, [x0, #-256]` | ACCEPT → 0xb810001e | ACCEPT → 0xf810001e | ACCEPT → 0xf810001e | differential |
| 140 | `ldtr d0, [x0]` | ACCEPT → 0xfc400800 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 141 | `stur sp, [x0, #-256]` | ACCEPT → 0xbc10001f | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 142 | `ldur v0, [x0]` | ACCEPT → 0xfc400000 | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 143 | `ldur x0, [w0]` | ACCEPT → 0xf8400000 | ERR: expected a 64-bit base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 144 | `stxp w0, w0, w0, [x0], x2` | ACCEPT → 0x88200000 | ERR: invalid addressing mode at operand 4 | ERR: invalid operand for instruction | silent-accept |
| 145 | `ldxp d0, x1, [x2]` | ACCEPT → 0x887f0440 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 146 | `ldxp x0, w1, [x2]` | ACCEPT → 0xc87f0440 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 147 | `stxp w0, w0, w0, [x0, #-1]` | ACCEPT → 0x88200000 | ERR: the optional immediate offset can only be 0 at operand 4 | ERR: index must be absent or #0 | silent-accept |
| 148 | `stxp w0, sp, w0, [x0]` | ACCEPT → 0xc820001f | ERR: expected an integer or zero register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 149 | `ldxp w0, w1, [w0]` | ACCEPT → 0x887f0400 | ERR: expected a 64-bit base register at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 150 | `stxp w0, w0, w0, [x0]` | ACCEPT → 0x88200000 | WARN: unpredictable: identical transfer and status registers | ERR: unpredictable STXP instruction, status is also a source | warning-class-CU |
| 151 | `stxp x0, x1, x2, [x3]` | ACCEPT → 0xc8200861 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 152 | `ldxp x0, x1, [xzr]` | ACCEPT → 0xc87f07e0 | ERR: invalid base register at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 153 | `fadd v0.2d, v0.2s, v0.2s` | ACCEPT → 0x4e60d400 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 154 | `fadd v0.2s, v0.2s, v0.2s, v0.2s` | ACCEPT → 0x0e20d400 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 155 | `fadd x0.2s, v0.2s, v0.2s` | ACCEPT → 0x0e20d400 | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 156 | `fadd v0.2s, v0, v0.2s` | ACCEPT → 0x0e20d400 | ERR: invalid use of vector register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 157 | `movq %sp, %rax` | ACCEPT → 0x48 89 e0 | ERR: operand type mismatch for `movq' | ERR: invalid operand for instruction | silent-accept |
| 158 | `stxr w0, w0, [x0], x2` | ACCEPT → 0x88007c00 | ERR: invalid addressing mode at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 159 | `ldxr d0, [x1]` | ACCEPT → 0x885f7c20 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 160 | `stxr w1, x0, [x2, #-1]` | ACCEPT → 0xc8017c40 | ERR: the optional immediate offset can only be 0 at operand 3 | ERR: index must be absent or #0 | silent-accept |
| 161 | `ldxr sp, [x0]` | ACCEPT → 0xc85f7c1f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 162 | `ldxr x0, [w1]` | ACCEPT → 0xc85f7c20 | ERR: expected a 64-bit base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 163 | `stxr w0, w0, [x0]` | ACCEPT → 0x88007c00 | WARN: unpredictable: identical transfer and status registers | ERR: unpredictable STXR instruction, status is also a source | warning-class-CU |
| 164 | `stxr x0, x1, [x2]` | ACCEPT → 0xc8007c41 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 165 | `ldxrb x0, [x1]` | ACCEPT → 0x085f7c20 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 166 | `ldxr x0, [xzr]` | ACCEPT → 0xc85f7fe0 | ERR: invalid base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 167 | `ands v0.8b, v0.8b, v0.8b` | ACCEPT → 0x2e201c00 | ERR: expected an integer or predicate register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 168 | `and w0, w0, w0, w0` | ACCEPT → 0x0a000000 | ERR: unexpected register in the immediate operand at operand 3 | ERR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31] | silent-accept |
| 169 | `and d0, x0, x0` | ACCEPT → 0x0a000000 | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 170 | `and w0, w0, x0` | ACCEPT → 0x0a000000 | ERR: operand mismatch | ERR: expected compatible register or logical immediate | silent-accept |
| 171 | `and v0.4s, v0.4s, v0.4s` | ACCEPT → 0x0e201c00 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 172 | `and v0.16b, v0.8b, v0.16b` | ACCEPT → 0x4e201c00 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 173 | `and w0, w0, w0, lsl #32` | ACCEPT → 0x0a008000 | ERR: shift amount out of range 0 to 31 at operand 3 | ERR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31] | silent-accept |
| 174 | `and wsp, w0, w0` | ACCEPT → 0x0a00001f | ERR: unexpected register in the immediate operand at operand 3 | ERR: expected compatible register or logical immediate | silent-accept |
| 175 | `and w0, w0, w0, lslx #0` | ACCEPT → 0x0a000000 | ERR: unexpected register in the immediate operand at operand 3 | ERR: unexpected token in argument list | silent-accept |
| 176 | `madd w0, w0, w0, w0, x0` | ACCEPT → 0x1b000000 | ERR: unexpected characters following instruction at operand 4 | ERR: invalid operand for instruction | silent-accept |
| 177 | `madd d0, x1, x2, x3` | ACCEPT → 0x1b020c20 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 178 | `madd w0, w0, w0, x0` | ACCEPT → 0x1b000000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 179 | `madd wsp, w0, w0, w0` | ACCEPT → 0x1b00001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 180 | `movk x0, #0, x0` | ACCEPT → 0xf2800000 | ERR: shift operator expected at operand 2 | ERR: expected 'lsl' with optional integer 0, 16, 32 or 48 | silent-accept |
| 181 | `movk d0, #0` | ACCEPT → 0x72800000 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 182 | `movk w0, #-1` | ACCEPT → 0x729fffe0 | ERR: immediate out of range | ERR: immediate must be an integer in range [0, 65535]. | silent-accept |
| 183 | `movk w0, #0, lsr #0` | ACCEPT → 0x72800000 | ERR: only 'LSL' shift is permitted at operand 2 | ERR: expected 'lsl' with optional integer 0 or 16 | silent-accept |
| 184 | `movk wsp, #0` | ACCEPT → 0x7280001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 185 | `movn x0, #0, x0` | ACCEPT → 0x92800000 | ERR: shift operator expected at operand 2 | ERR: expected 'lsl' with optional integer 0, 16, 32 or 48 | silent-accept |
| 186 | `movn d0, #0` | ACCEPT → 0x12800000 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 187 | `movn w0, #-1` | ACCEPT → 0x129fffe0 | ERR: immediate out of range | ERR: immediate must be an integer in range [0, 65535]. | silent-accept |
| 188 | `movn w0, #0, lsr #0` | ACCEPT → 0x12800000 | ERR: only 'LSL' shift is permitted at operand 2 | ERR: expected 'lsl' with optional integer 0 or 16 | silent-accept |
| 189 | `movn wsp, #0` | ACCEPT → 0x1280001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 190 | `movz x0, #0, x0` | ACCEPT → 0xd2800000 | ERR: shift operator expected at operand 2 | ERR: expected 'lsl' with optional integer 0, 16, 32 or 48 | silent-accept |
| 191 | `movz d0, #0` | ACCEPT → 0x52800000 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 192 | `movz w0, #-1` | ACCEPT → 0x529fffe0 | ERR: immediate out of range | ERR: immediate must be an integer in range [0, 65535]. | silent-accept |
| 193 | `movz w0, #0, lsr #0` | ACCEPT → 0x52800000 | ERR: only 'LSL' shift is permitted at operand 2 | ERR: expected 'lsl' with optional integer 0 or 16 | silent-accept |
| 194 | `movz wsp, #0` | ACCEPT → 0x5280001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 195 | `sqshrn v0.8b, v0.8h, #1, v0.8b` | ACCEPT → 0x0f0f9400 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 196 | `sqshrn x0, v0.8h, #1` | ERR: expected register | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | real — parser-masked at CLI (unit-proven: encoder accepts GPR dest) |
| 197 | `sqshrn v0.4h, v0.8h, #1` | ACCEPT → 0x0f0f9400 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 198 | `sqshrn v0.8h, v0.8b, #4294967297` | ERR: qshrn: unsupported source: 8b | ERR: immediate value out of range 1 to 64 at operand 3 | ERR: invalid operand for instruction | real — parser-masked at CLI (encoder truncates via as u32 per issue; unit spot-check pending) |
| 199 | `sqshrn v0.8b, v0.8h, #9` | ACCEPT → 0x0f079400 | ERR: immediate value out of range 1 to 8 at operand 3 | ERR: immediate must be an integer in range [1, 8]. | silent-accept |
| 200 | `msub w0, w0, w0, w0, x0` | ACCEPT → 0x1b008000 | ERR: unexpected characters following instruction at operand 4 | ERR: invalid operand for instruction | silent-accept |
| 201 | `msub d0, x1, x2, x3` | ACCEPT → 0x1b028c20 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 202 | `msub w0, w0, w0, x0` | ACCEPT → 0x1b008000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 203 | `msub wsp, w0, w0, w0` | ACCEPT → 0x1b00801f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 204 | `mul w0, w0, w0, x0` | ACCEPT → 0x1b007c00 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 205 | `mul d0, x1, x2` | ACCEPT → 0x1b027c20 | ERR: expected an integer or vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 206 | `mul w0, w0, x0` | ACCEPT → 0x1b007c00 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 207 | `mul v0.2d, v0.2d, v0.2d` | ACCEPT → 0x4ee09c00 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 208 | `mul v0.4h, v0.8b, v0.4h` | ACCEPT → 0x0e609c00 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 209 | `mul wsp, w0, w0` | ACCEPT → 0x1b007c1f | ERR: expected an integer or vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 210 | `mvn w0, w0, foo #0` | ACCEPT → 0x2a2003e0 | ERR: shift operator expected at operand 2 | ERR: unexpected token in argument list | silent-accept |
| 211 | `mvn w0, w0, x0` | ACCEPT → 0x2a2003e0 | ERR: shift operator expected at operand 2 | ERR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31] | silent-accept |
| 212 | `mvn d0, x1` | ACCEPT → 0x2a2103e0 | ERR: expected an integer register or Advanced SIMD vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 213 | `mvn w0, x0` | ACCEPT → 0x2a2003e0 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 214 | `mvn v0.16b, v0.16b, x0` | ACCEPT → 0x6e205800 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 215 | `mvn v0.16b, v0.8b` | ACCEPT → 0x6e205800 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 216 | `mvn v0.4h, v0.4h` | ACCEPT → 0x2e205800 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 217 | `mvn w0, w0, lsl #32` | ACCEPT → 0x2a2083e0 | ERR: shift amount out of range 0 to 31 at operand 2 | ERR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31] | silent-accept |
| 218 | `mvn wsp, w0` | ACCEPT → 0x2a2003ff | ERR: expected an integer register or Advanced SIMD vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 219 | `srshr v0.8b, v0.8b, #1, v0.8b` | ACCEPT → 0x0f0f2400 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 220 | `srshr v0.8b, v0.16b, #1` | ACCEPT → 0x0f0f2400 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 221 | `sshr v0.8b, x0, #1` | ACCEPT → 0x0f0f0400 | ERR: expected an Advanced SIMD vector register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 222 | `sshr v0.8b, v0.8b, #4294967297` | ACCEPT → 0x0f0f0400 | ERR: immediate value out of range 1 to 64 at operand 3 | ERR: immediate must be an integer in range [1, 8]. | silent-accept |
| 223 | `neg w0, w0, w0` | ACCEPT → 0x4b0003e0 | ERR: shift operator expected at operand 2 | ERR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31] | silent-accept |
| 224 | `negs w0, w0, ror #0` | ACCEPT → 0x6b0003e0 | ERR: 'ROR' operator not allowed at operand 2 | ERR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31] | silent-accept |
| 225 | `negs w0, w0, x0` | ACCEPT → 0x6b0003e0 | ERR: shift operator expected at operand 2 | ERR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31] | silent-accept |
| 226 | `negs d0, x1` | ACCEPT → 0x6b0103e0 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 227 | `negs w0, x0` | ACCEPT → 0x6b0003e0 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 228 | `negs w0, w0, lsl #32` | ACCEPT → 0x6b0083e0 | ERR: shift amount out of range 0 to 31 at operand 2 | ERR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31] | silent-accept |
| 229 | `negs wsp, w0` | ACCEPT → 0x6b0003ff | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 230 | `ushr v0.8b, v0.8b, #1, v0.8b` | ACCEPT → 0x2f0f0400 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 231 | `ushr v0.8b, v0.16b, #1` | ACCEPT → 0x2f0f0400 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 232 | `ushr v0.8b, x0, #1` | ACCEPT → 0x2f0f0400 | ERR: expected an Advanced SIMD vector register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 233 | `ushr v0.8b, v0.8b, #4294967297` | ACCEPT → 0x2f0f0400 | ERR: immediate value out of range 1 to 64 at operand 3 | ERR: immediate must be an integer in range [1, 8]. | silent-accept |
| 234 | `ushr v0.8b, v0.8b, #-1` | PANIC | ERR: immediate value out of range 1 to 64 at operand 3 | ERR: immediate must be an integer in range [1, 8]. | panic-on-invalid |
| 235 | `tbl v0.8b, {v1.16b}, x0` | ACCEPT → 0x0e000020 | ERR: expected an Advanced SIMD vector register at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 236 | `tbl v0.8b, {v0.16b}, x0` | ACCEPT → 0x0e000000 | ERR: expected an Advanced SIMD vector register at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 237 | `tbl v0.8b, {}, v0.8b` | ERR: Line 1: empty register list: 'tbl v0.8b, {}, v0.8b' | ERR: syntax error in register list at operand 2 | ERR: vector register expected | real — parser-masked at CLI (unit-proven: regs[0] panic) |
| 238 | `tbl v0.8b, {v0.16b}, v0.8b, v0.8b` | ACCEPT → 0x0e000000 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 239 | `tbl v0.8b, {v0.16b, v1.16b, v2.16b, v3.16b, v4.16b}, v0.8b` | ACCEPT → 0x0e000000 | ERR: too many registers in vector register list at operand 2 | ERR: invalid number of vectors | silent-accept |
| 240 | `tbl x0, {v0.16b}, v0.8b` | ACCEPT → 0x0e000000 | ERR: expected a vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 241 | `tbl v0.4h, {v0.16b}, v0.4h` | ACCEPT → 0x0e000000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 242 | `tbl v0.8b, {v0.16b}, v0.16b` | ACCEPT → 0x0e000000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 243 | `tbl v0.8b, {v0.16b, v2.16b}, v0.8b` | ACCEPT → 0x0e002000 | ERR: the register list must have a stride of 1 at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 244 | `tbl v0.8b, {v0.8b}, v0.8b` | ACCEPT → 0x0e000000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 245 | `orn w0, w0, w0, w0` | ACCEPT → 0x2a200000 | ERR: shift operator expected at operand 3 | ERR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31] | silent-accept |
| 246 | `orn d0, x1, x2` | ACCEPT → 0x2a220020 | ERR: expected an integer, vector or predicate register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 247 | `orn w0, w0, #0xaaaaaaaa` | ERR: expected register at operand 2, got Some(Imm(2863311530)) | ERR: integer register expected in the extended/shifted operand register at operand 3 | ACCEPT → 0x3200f000 | not-a-defect (gas parity): llvm-mc-only alias — enhancement request |
| 248 | `orn v0.8h, v0.8h, v0.8h` | ACCEPT → 0x0ee01c00 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 249 | `orn w0, w0, x0` | ACCEPT → 0x2a200000 | ERR: operand mismatch | ERR: expected compatible register or logical immediate | silent-accept |
| 250 | `orn v0.8b, v0.16b, v0.8b` | ACCEPT → 0x0ee01c00 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 251 | `orn w0, w0, w0, lsl #32` | ACCEPT → 0x2a208000 | ERR: shift amount out of range 0 to 31 at operand 3 | ERR: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31] | silent-accept |
| 252 | `orn wsp, w0, w0` | ACCEPT → 0x2a20001f | ERR: expected an integer, vector or predicate register at operand 1 | ERR: expected compatible register or logical immediate | silent-accept |
| 253 | `orn w0, w0, w0, lsl #1, w0` | ACCEPT → 0x2a200400 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 254 | `orn w0, w0, w0, lslx #0` | ACCEPT → 0x2a200000 | ERR: shift operator expected at operand 3 | ERR: unexpected token in argument list | silent-accept |
| 255 | `ret x0, x1` | ACCEPT → 0xd65f0000 | ERR: unexpected characters following instruction at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 256 | `ret d0` | ACCEPT → 0xd65f0000 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 257 | `ret sp` | ACCEPT → 0xd65f03e0 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 258 | `ret w0` | ACCEPT → 0xd65f0000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 259 | `sbc w0, w0, w0, lsl #0` | ACCEPT → 0x5a000000 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 260 | `sbc d0, x1, x2` | ACCEPT → 0x5a020020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 261 | `sbc w0, w0, x0` | ACCEPT → 0x5a000000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 262 | `sbc wsp, w0, w0` | ACCEPT → 0x5a00001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 263 | `shlw $1, %al ; shlq $1, %xmm0` | ACCEPT → 0x66 d1 e0 | ERR: `%al' not allowed with `shlw' | ERR: invalid operand for instruction | silent-accept |
| 264 | `shlb $0, %fs:(%rax)` | ACCEPT → 0xc0 20 00 | ACCEPT → 0x64 c0 20 00 | ACCEPT → 0x64 c0 20 00 | differential |
| 265 | `shlb $256, %al` | ACCEPT → 0xc0 e0 00 | WARN: /tmp/fullver/p265.s:1: Warning: 0x100 shortened to 0x0 | ERR: invalid operand for instruction | warning-class-CU |
| 266 | `sshll v0.8h, v0.8b, #0, v0.8h` | ACCEPT → 0x0f08a400 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 267 | `sshll x0, v0.8b, #0` | ACCEPT → 0x0f08a400 | ERR: expected an Advanced SIMD vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 268 | `sshll v0.8b, v0.8b, #0` | ACCEPT → 0x0f08a400 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 269 | `sshll v0.8h, v0.8b, #8` | ACCEPT → 0x0f10a400 | ERR: immediate value out of range 0 to 7 at operand 3 | ERR: immediate must be an integer in range [0, 7]. | silent-accept |
| 270 | `sshll2 v0.8h, v0.8b, #0` | ACCEPT → 0x4f08a400 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 271 | `sqshrun v0.8b, v0.8h, #1, v0.8b` | ACCEPT → 0x2f0f8400 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 272 | `sqshrun x0, v0.8h, #1` | ACCEPT → 0x2f0f8400 | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 273 | `sqshrun v0.4h, v0.8h, #1` | ACCEPT → 0x2f0f8400 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 274 | `sqshrun v0.8b, v0.8h, #4294967297` | ACCEPT → 0x2f0f8400 | ERR: immediate value out of range 1 to 64 at operand 3 | ERR: immediate must be an integer in range [1, 8]. | silent-accept |
| 275 | `sqshrun v0.8b, v0.8h, #9` | ACCEPT → 0x2f0f8400 | ERR: immediate value out of range 1 to 8 at operand 3 | ERR: immediate must be an integer in range [1, 8]. | silent-accept |
| 276 | `smull x0, w0, w0, x0` | ACCEPT → 0x9b207c00 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 277 | `smull d0, w1, w2` | ACCEPT → 0x9b227c20 | ERR: expected an integer register or Advanced SIMD vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 278 | `smull wsp, w0, w0` | ACCEPT → 0x9b207c1f | ERR: expected an integer register or Advanced SIMD vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 279 | `smull w0, w0, w0` | ACCEPT → 0x9b207c00 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 280 | `uxtb w0, w0, w0` | ACCEPT → 0x53001c00 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 281 | `sxth d0, w1` | ACCEPT → 0x13003c20 | ERR: expected an integer register or SVE vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 282 | `uxth w0, w0, w0` | ACCEPT → 0x53003c00 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 283 | `sxth w0, x0` | ACCEPT | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 284 | `sxtw x0, w0, x0` | ACCEPT → 0x93407c00 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 285 | `sxtb w0, w0, w0` | ACCEPT → 0x13001c00 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 286 | `sxth w0, w0, w0` | ACCEPT → 0x13003c00 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 287 | `sxtw w0, x0` | ACCEPT | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 288 | `sqshl v0.8b, v0.16b, #0` | ACCEPT → 0x0f087400 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 289 | `sqshl v0.8b, v0.8b, #0, v0.8b` | ACCEPT → 0x0f087400 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 290 | `sqshl x0.8b, v0.8b, #0` | ACCEPT → 0x0f087400 | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 291 | `sqshl v0.8b, v0.8b, #-1` | PANIC | ERR: immediate value out of range 0 to 7 at operand 3 | ERR: immediate must be an integer in range [0, 7]. | panic-on-invalid |
| 292 | `sqshl v0.8b, v0, #0` | ACCEPT → 0x0f087400 | ERR: invalid use of vector register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 293 | `umaddl x0, w0, w0, x0, x0` | ACCEPT → 0x9ba00000 | ERR: unexpected characters following instruction at operand 4 | ERR: invalid operand for instruction | silent-accept |
| 294 | `umaddl d0, w1, w2, x3` | ACCEPT → 0x9ba20c20 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 295 | `umaddl wsp, w0, w0, x0` | ACCEPT → 0x9ba0001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 296 | `umaddl w0, w0, w0, w0` | ACCEPT → 0x9ba00000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 297 | `umulh x0, x0, x0, x0` | ACCEPT → 0x9bc07c00 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 298 | `umulh d0, x1, x2` | ACCEPT → 0x9bc27c20 | ERR: expected an integer register or SVE vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 299 | `umulh wsp, x0, x0` | ACCEPT → 0x9bc07c1f | ERR: expected an integer register or SVE vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 300 | `umulh w0, w0, w0` | ACCEPT → 0x9bc07c00 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 301 | `rbit v0.8b, v0` | ACCEPT → 0x2e605800 | ERR: invalid use of vector register at operand 2 | ERR: too few operands for instruction | silent-accept |
| 302 | `rbit v0.8b, v0.8b, v0.8b` | ACCEPT → 0x2e605800 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 303 | `rbit v0.8b, v0.16b` | ACCEPT → 0x2e605800 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 304 | `rbit x0.8b, x0.8b` | ACCEPT → 0x2e605800 | ERR: comma expected between operands at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 305 | `rbit sp.8b, v0.8b` | ACCEPT → 0x2e60581f | ERR: expected an integer or vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 306 | `umull x0, w0, w0, x0` | ACCEPT → 0x9ba07c00 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 307 | `umull d0, w1, w2` | ACCEPT → 0x9ba27c20 | ERR: expected an integer register or Advanced SIMD vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 308 | `umull wsp, w0, w0` | ACCEPT → 0x9ba07c1f | ERR: expected an integer register or Advanced SIMD vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 309 | `umull w0, w0, w0` | ACCEPT → 0x9ba07c00 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 310 | `uxtw x0, w0, x0` | ACCEPT → 0x2a0003e0 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 311 | `uxtw d0, w1` | ACCEPT → 0x2a0103e0 | ERR: expected an integer register or SVE vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 312 | `uxtw x0, w0, w0` | ACCEPT → 0x2a0003e0 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 313 | `uxtw x0, w0, x0` | ACCEPT → 0x2a0003e0 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 314 | `uxtw d0, w0` | ACCEPT → 0x2a0003e0 | ERR: expected an integer register or SVE vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 315 | `stlxr w0, w0, [x0], x2` | ACCEPT → 0x8800fc00 | ERR: invalid addressing mode at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 316 | `ldaxr d0, [x1]` | ACCEPT → 0x885ffc20 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 317 | `stlxr w1, x0, [x2, #-1]` | ACCEPT → 0xc801fc40 | ERR: the optional immediate offset can only be 0 at operand 3 | ERR: index must be absent or #0 | silent-accept |
| 318 | `ldaxr sp, [x0]` | ACCEPT → 0xc85ffc1f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 319 | `ldaxr x0, [w1]` | ACCEPT → 0xc85ffc20 | ERR: expected a 64-bit base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 320 | `stlxr w0, w0, [x0]` | ACCEPT → 0x8800fc00 | WARN: unpredictable: identical transfer and status registers | ERR: unpredictable STXR instruction, status is also a source | warning-class-CU |
| 321 | `stlxr x0, x1, [x2]` | ACCEPT → 0xc800fc41 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 322 | `ldaxrb x0, [x1]` | ACCEPT → 0x085ffc20 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 323 | `ldaxr x0, [xzr]` | ACCEPT → 0xc85fffe0 | ERR: invalid base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 324 | `ldrsw x0, [x1, #0], x2` | ACCEPT → 0xb9800020 | ERR: cannot combine pre- and post-indexing at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 325 | `ldrsw d0, [x1, #0]` | ACCEPT → 0xb9800020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 326 | `ldrsw x0, foo` | ERR: unsupported ldrsw operands: [Reg("x0"), Symbol("foo")] | ACCEPT → 0x98000000 | ACCEPT → 0x98000000 | reverse |
| 327 | `ldrsw x0, [x1, #-257]` | ACCEPT → 0xb88ff020 | ERR: immediate offset out of range | ERR: index must be an integer in range [-256, 255]. | silent-accept |
| 328 | `ldrsw sp, [x1, #0]` | ACCEPT → 0xb980003f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 329 | `ldrsw x0, [w1]` | ACCEPT → 0xb9800020 | ERR: expected a 64-bit base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 330 | `ldrsw w0, [x1, #0]` | ACCEPT → 0xb9800020 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 331 | `ldrsw x0, [x1, w2]` | ACCEPT → 0xb8a26820 | ERR: invalid use of 32-bit register offset at operand 2 | ERR: expected 'uxtw' or 'sxtw' with optional shift of #0 or #2 | silent-accept |
| 332 | `ldrsw x0, [x0, #4]!` | ACCEPT → 0xb8804c00 | WARN: /tmp/fullver/p332.s:1: Warning: unpredictable transfer with writeback | ERR: unpredictable LDR instruction, writeback base is also a source | warning-class-CU |
| 333 | `ldrsw x0, [xzr]` | ACCEPT → 0xb98003e0 | ERR: invalid base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 334 | `sttrb w0, [x0, #-256], x2` | ACCEPT → 0x38100800 | ERR: cannot combine pre- and post-indexing at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 335 | `ldtrb d0, [x0]` | ACCEPT → 0x38400800 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 336 | `sttrb w0, [x0, #-257]` | ACCEPT → 0x380ff800 | ERR: immediate offset out of range -256 to 255 at operand 2 | ERR: index must be an integer in range [-256, 255]. | silent-accept |
| 337 | `sttrb sp, [x0, #-256]` | ACCEPT → 0x3810081f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 338 | `ldtrb w0, [w0]` | ACCEPT → 0x38400800 | ERR: expected a 64-bit base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 339 | `sttrb x0, [x0, #-256]` | ACCEPT → 0x38100800 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 340 | `ldtrb w0, [xzr]` | ACCEPT → 0x38400be0 | ERR: invalid base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 341 | `prfm pldl1keep, [x0, x1, lsl #1]` | ACCEPT → 0xf9217800 | ERR: invalid shift amount at operand 2 | ERR: expected 'lsl' or 'sxtx' with optional shift of #0 or #3 | silent-accept |
| 342 | `prfm pldl1keep, [x0], x2` | ACCEPT → 0xf9800000 | ERR: invalid addressing mode at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 343 | `prfm pldl1keep, [d0]` | ACCEPT → 0xf9800000 | ERR: invalid base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 344 | `prfm pldl1keep, [x0, x1]` | ACCEPT → 0xf9216800 | ACCEPT → 0xf8a16800 | ACCEPT → 0xf8a16800 | differential |
| 345 | `prfm #0, [w0]` | ACCEPT → 0xf9800000 | ERR: expected a 64-bit base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 346 | `prfm pldl1keep, [x0, w0]` | ACCEPT → 0xf9204800 | ERR: invalid use of 32-bit register offset at operand 2 | ERR: expected 'uxtw' or 'sxtw' with optional shift of #0 or #3 | silent-accept |
| 347 | `prfm pldl1keep, [xzr]` | ACCEPT → 0xf98003e0 | ERR: invalid base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 348 | `smulh x0, x0, x0, x0` | ACCEPT → 0x9b407c00 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 349 | `smulh d0, x1, x2` | ACCEPT → 0x9b427c20 | ERR: expected an integer register or SVE vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 350 | `smulh wsp, x0, x0` | ACCEPT → 0x9b407c1f | ERR: expected an integer register or SVE vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 351 | `smulh w0, w0, w0` | ACCEPT → 0x9b407c00 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 352 | `smulh wzr, x0, x0` | ACCEPT → 0x9b407c1f | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 353 | `fcvtzs w0, s0, x0` | ACCEPT → 0x1e380000 | ERR: immediate operand required at operand 3 | ERR: immediate must be an integer in range [1, 32]. | silent-accept |
| 354 | `fcvtzs s0, s0` | ACCEPT → 0x1e380000 | ACCEPT → 0x5ea1b800 | ACCEPT → 0x5ea1b800 | differential |
| 355 | `fcvtzs w0, h0` | ACCEPT → 0x1e380000 | ACCEPT → 0x1ef80000 | ACCEPT → 0x1ef80000 | differential |
| 356 | `fcvtzs wsp, s0` | ACCEPT → 0x1e38001f | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 357 | `fabs s0, s0, s0` | ACCEPT → 0x1e20c000 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 358 | `frintn h0, h0` | ACCEPT → 0x1e244000 | ACCEPT → 0x1ee44000 | ACCEPT → 0x1ee44000 | differential |
| 359 | `fabs s0, d0` | ACCEPT → 0x1e20c000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 360 | `ucvtf s0, w0, x0` | ACCEPT → 0x1e230000 | ERR: immediate operand required at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 361 | `ucvtf h0, w0` | ACCEPT → 0x1e230000 | ACCEPT → 0x1ee30000 | ACCEPT → 0x1ee30000 | differential |
| 362 | `scvtf s0, wsp` | ACCEPT → 0x1e2203e0 | ERR: unexpected register type at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 363 | `ucvtf w0, w0` | ACCEPT → 0x1e230000 | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 364 | `fcmp s0` | ACCEPT → 0x1e202008 | ERR: comma expected between operands at operand 2 | ERR: too few operands for instruction | silent-accept |
| 365 | `fcmp s0, s0, s0` | ACCEPT → 0x1e202000 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 366 | `fcmp h0, h0` | ACCEPT → 0x1e202000 | ACCEPT → 0x1ee02000 | ACCEPT → 0x1ee02000 | differential |
| 367 | `fcmp s0, d0` | ACCEPT → 0x1e202000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 368 | `fcvt s0, d0, s0` | ACCEPT → 0x1e624000 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 369 | `fcvt s0, s0` | ACCEPT → 0x1e224000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 370 | `fcvt sp, s0` | ACCEPT → 0x1e22401f | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 371 | `aese v0.8b, v0.8b` | ACCEPT → 0x4e284800 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 372 | `aese v0.16b, v0` | ACCEPT → 0x4e284800 | ERR: invalid use of vector register at operand 2 | ERR: too few operands for instruction | silent-accept |
| 373 | `aese v0.16b, v0.16b, v0.16b` | ACCEPT → 0x4e284800 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 374 | `aese v0.16b, v0.8b` | ACCEPT → 0x4e284800 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 375 | `aese x0.16b, x0.16b` | ACCEPT → 0x4e284800 | ERR: expected a vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 376 | `aese sp.16b, v0.16b` | ACCEPT → 0x4e28481f | ERR: expected a vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 377 | `bfi w0, w0, #0, #1, x0` | ACCEPT → 0x33000000 | ERR: unexpected characters following instruction at operand 4 | ERR: unrecognized instruction mnemonic, did you mean: b, bfm, bic, bif, bit, bti, wfi? | silent-accept |
| 378 | `bfi d0, x1, #0, #1` | ACCEPT → 0x33000020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 379 | `bfi w0, w0, #0, #0` | PANIC | ERR: immediate value out of range 1 to 32 at operand 4 | ERR: expected integer in range [1, 32] | panic-on-invalid |
| 380 | `bfi x0, w0, #0, #1` | ACCEPT → 0xb3400000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 381 | `bfi wsp, w0, #0, #1` | ACCEPT → 0x3300001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 382 | `bfxil w0, w0, #0, #1, x0` | ACCEPT → 0x33000000 | ERR: unexpected characters following instruction at operand 4 | ERR: unrecognized instruction mnemonic | silent-accept |
| 383 | `bfxil d0, x1, #0, #1` | ACCEPT → 0x33000020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 384 | `bfxil w0, w0, #0, #0` | PANIC | ERR: immediate value out of range 1 to 32 at operand 4 | ERR: expected integer in range [1, 32] | panic-on-invalid |
| 385 | `bfxil x0, w0, #0, #1` | ACCEPT → 0xb3400000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 386 | `bfxil wsp, w0, #0, #1` | ACCEPT → 0x3300001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 387 | `casb x0, x1, [x2]` | ACCEPT → 0x08a07c41 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 388 | `cas w0, w0, [x0], x2` | ACCEPT → 0x88a07c00 | ERR: invalid addressing mode at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 389 | `cas s0, s1, [x2]` | ACCEPT → 0x88a07c41 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 390 | `cas x0, w0, [x1]` | ACCEPT → 0xc8a07c20 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 391 | `cas w0, w0, [x0, #-1]` | ACCEPT → 0x88a07c00 | ERR: the optional immediate offset can only be 0 at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 392 | `cas sp, w1, [x2]` | ACCEPT → 0xc8bf7c41 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 393 | `cas w0, w1, [w2]` | ACCEPT → 0x88a07c41 | ERR: expected a 64-bit base register at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 394 | `cas x0, x1, [xzr]` | ACCEPT → 0xc8a07fe1 | ERR: invalid base register at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 395 | `cls w0, w0, x0` | ACCEPT → 0x5ac01400 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 396 | `cls d0, x1` | ACCEPT → 0x5ac01420 | ERR: expected an integer or vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 397 | `cls x0, w0` | ACCEPT → 0xdac01400 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 398 | `cls wsp, w0` | ACCEPT → 0x5ac0141f | ERR: expected an integer or vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 399 | `clz w0, w0, x0` | ACCEPT → 0x5ac01000 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 400 | `clz d0, x1` | ACCEPT → 0x5ac01020 | ERR: expected an integer or vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 401 | `clz x0, w0` | ACCEPT → 0xdac01000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 402 | `clz wsp, w0` | ACCEPT → 0x5ac0101f | ERR: expected an integer or vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 403 | `extr w0, w0, w0, #0, x0` | ACCEPT → 0x13800000 | ERR: unexpected characters following instruction at operand 4 | ERR: invalid operand for instruction | silent-accept |
| 404 | `extr d0, x1, x2, #0` | ACCEPT → 0x13820020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 405 | `extr w0, w0, w0, #-1` | ACCEPT → 0xfffffc00 | ERR: immediate value out of range 0 to 63 at operand 4 | ERR: immediate must be an integer in range [0, 31]. | silent-accept |
| 406 | `extr w0, x0, w0, #0` | ACCEPT → 0x13800000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 407 | `extr wsp, w0, w0, #0` | ACCEPT → 0x1380001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 408 | `fmov s0, s0, s0` | ACCEPT → 0x1e204000 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 409 | `fmov h0, h0` | ACCEPT → 0x1e204000 | ACCEPT → 0x1ee04000 | ACCEPT → 0x1ee04000 | differential |
| 410 | `fmov wsp, s0` | ACCEPT → 0x1e26001f | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 411 | `fmov x0, v0.d[1]` | ERR: fmov needs register operands | ACCEPT → 0x9eae0000 | ACCEPT → 0x9eae0000 | reverse |
| 412 | `fmov q0, s0` | ACCEPT → 0x1e204000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 413 | `fadd s0, s0, s0, s0` | ACCEPT → 0x1e202800 | ERR: unexpected characters following instruction at operand 3 | ERR: invalid operand for instruction | silent-accept |
| 414 | `fmul h0, h0, h0` | ACCEPT → 0x1e200800 | ACCEPT → 0x1ee00800 | ACCEPT → 0x1ee00800 | differential |
| 415 | `fsub d0, s0, s0` | ACCEPT → 0x1e603800 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 416 | `rbit w0, w0, x0` | ACCEPT → 0x5ac00000 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 417 | `rbit d0, x1` | ACCEPT → 0x5ac00020 | ERR: expected an integer or vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 418 | `rbit x0, w0` | ACCEPT → 0xdac00000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 419 | `rbit wsp, w0` | ACCEPT → 0x5ac0001f | ERR: expected an integer or vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 420 | `rev16 w0, w0, x0` | ACCEPT → 0x5ac00400 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 421 | `rev16 d0, x1` | ACCEPT → 0x5ac00420 | ERR: expected an integer register or Advanced SIMD vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 422 | `rev16 x0, w0` | ACCEPT → 0xdac00400 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 423 | `rev16 wsp, w0` | ACCEPT → 0x5ac0041f | ERR: expected an integer register or Advanced SIMD vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 424 | `rev32 x0, x0, x0` | ACCEPT → 0xdac00800 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 425 | `rev32 d0, x1` | ACCEPT → 0xdac00820 | ERR: expected an integer register or Advanced SIMD vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 426 | `rev32 x0, w0` | ACCEPT → 0xdac00800 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 427 | `rev32 v0.2s, v1.2s` | ACCEPT → 0x2ea00820 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 428 | `rev32 wsp, x0` | ACCEPT → 0xdac0081f | ERR: expected an integer register or Advanced SIMD vector register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 429 | `rev32 w0, w0` | ACCEPT → 0xdac00800 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 430 | `rev w0, w0, x0` | ACCEPT → 0x5ac00800 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 431 | `rev d0, x1` | ACCEPT → 0x5ac00820 | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 432 | `rev x0, w0` | ACCEPT → 0xdac00c00 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 433 | `rev wsp, w0` | ACCEPT → 0x5ac0081f | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 434 | `sbfiz w0, w0, #0, #1, x0` | ACCEPT → 0x13000000 | ERR: unexpected characters following instruction at operand 4 | ERR: unrecognized instruction mnemonic | silent-accept |
| 435 | `sbfiz d0, x1, #0, #1` | ACCEPT → 0x13000020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 436 | `sbfiz w0, w0, #0, #0` | PANIC | ERR: immediate value out of range 1 to 32 at operand 4 | ERR: expected integer in range [1, 32] | panic-on-invalid |
| 437 | `sbfiz x0, w0, #0, #1` | ACCEPT → 0x93400000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 438 | `sbfiz wsp, w0, #0, #1` | ACCEPT → 0x1300001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 439 | `ubfiz w0, w0, #0, #1, x0` | ACCEPT → 0x53000000 | ERR: unexpected characters following instruction at operand 4 | ERR: unrecognized instruction mnemonic | silent-accept |
| 440 | `ubfiz d0, x1, #0, #1` | ACCEPT → 0x53000020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 441 | `ubfiz w0, w0, #0, #0` | PANIC | ERR: immediate value out of range 1 to 32 at operand 4 | ERR: expected integer in range [1, 32] | panic-on-invalid |
| 442 | `ubfiz x0, w0, #0, #1` | ACCEPT → 0xd3400000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 443 | `ubfiz wsp, w0, #0, #1` | ACCEPT → 0x5300001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 444 | `bfm w0, w0, #0, #0, x0` | ACCEPT → 0x33000000 | ERR: unexpected characters following instruction at operand 4 | ERR: invalid operand for instruction | silent-accept |
| 445 | `bfm d0, x1, #0, #0` | ACCEPT → 0x33000020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 446 | `bfm w0, w0, #-1, #0` | ACCEPT → 0xffff0000 | ERR: immediate value out of range 0 to 63 at operand 3 | ERR: immediate must be an integer in range [0, 31]. | silent-accept |
| 447 | `bfm x0, w0, #0, #0` | ACCEPT → 0xb3400000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 448 | `bfm wsp, w0, #0, #0` | ACCEPT → 0x3300001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 449 | `sbfm w0, w0, #0, #0, x0` | ACCEPT → 0x13000000 | ERR: unexpected characters following instruction at operand 4 | ERR: invalid operand for instruction | silent-accept |
| 450 | `sbfm d0, x1, #0, #0` | ACCEPT → 0x13000020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 451 | `sbfm w0, w0, #-1, #0` | ACCEPT → 0xffff0000 | ERR: immediate value out of range 0 to 63 at operand 3 | ERR: immediate must be an integer in range [0, 31]. | silent-accept |
| 452 | `sbfm x0, w0, #0, #0` | ACCEPT → 0x93400000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 453 | `sbfm wsp, w0, #0, #0` | ACCEPT → 0x1300001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 454 | `sbfx w0, w0, #0, #1, x0` | ACCEPT → 0x13000000 | ERR: unexpected characters following instruction at operand 4 | ERR: unrecognized instruction mnemonic, did you mean: sbfm? | silent-accept |
| 455 | `sbfx d0, x1, #0, #1` | ACCEPT → 0x13000020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 456 | `sbfx w0, w0, #0, #0` | PANIC | ERR: immediate value out of range 1 to 32 at operand 4 | ERR: expected integer in range [1, 32] | panic-on-invalid |
| 457 | `sbfx x0, w0, #0, #1` | ACCEPT → 0x93400000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 458 | `sbfx wsp, w0, #0, #1` | ACCEPT → 0x1300001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 459 | `ubfm w0, w0, #0, #0, x0` | ACCEPT → 0x53000000 | ERR: unexpected characters following instruction at operand 4 | ERR: invalid operand for instruction | silent-accept |
| 460 | `ubfm d0, x1, #0, #0` | ACCEPT → 0x53000020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 461 | `ubfm w0, w0, #-1, #0` | ACCEPT → 0xffff0000 | ERR: immediate value out of range 0 to 63 at operand 3 | ERR: immediate must be an integer in range [0, 31]. | silent-accept |
| 462 | `ubfm x0, w0, #0, #0` | ACCEPT → 0xd3400000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 463 | `ubfm wsp, w0, #0, #0` | ACCEPT → 0x5300001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 464 | `ubfx w0, w0, #0, #1, x0` | ACCEPT → 0x53000000 | ERR: unexpected characters following instruction at operand 4 | ERR: unrecognized instruction mnemonic, did you mean: ubfm? | silent-accept |
| 465 | `ubfx d0, x1, #0, #1` | ACCEPT → 0x53000020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 466 | `ubfx w0, w0, #0, #0` | PANIC | ERR: immediate value out of range 1 to 32 at operand 4 | ERR: expected integer in range [1, 32] | panic-on-invalid |
| 467 | `ubfx x0, w0, #0, #1` | ACCEPT → 0xd3400000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 468 | `ubfx wsp, w0, #0, #1` | ACCEPT → 0x5300001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 469 | `fabs v0.2s, v0.4s` | ACCEPT → 0x0ea0f800 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 470 | `fabs v0.2s, v0` | ACCEPT → 0x0ea0f800 | ERR: invalid use of vector register at operand 2 | ERR: too few operands for instruction | silent-accept |
| 471 | `fabs v0.2s, v0.2s, v0.2s` | ACCEPT → 0x0ea0f800 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 472 | `fabs x0.2s, v0.2s` | ACCEPT → 0x0ea0f800 | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 473 | `fabs sp.2s, v0.2s` | ACCEPT → 0x0ea0f81f | ERR: unexpected register type at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 474 | `fabs s0, s0, s0` | ACCEPT → 0x1e20c000 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 475 | `fabs h0, h0` | ACCEPT → 0x1e20c000 | ACCEPT → 0x1ee0c000 | ACCEPT → 0x1ee0c000 | differential |
| 476 | `fabs s0, d0` | ACCEPT → 0x1e20c000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 477 | `fmadd s0, s0, s0, s0, s0` | ACCEPT → 0x1f000000 | ERR: unexpected characters following instruction at operand 4 | ERR: invalid operand for instruction | silent-accept |
| 478 | `fmadd h0, h0, h0, h0` | ACCEPT → 0x1f000000 | ACCEPT → 0x1fc00000 | ACCEPT → 0x1fc00000 | differential |
| 479 | `fmadd d0, s0, s0, s0` | ACCEPT → 0x1f400000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 480 | `fneg s0, s0, s0` | ACCEPT → 0x1e214000 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 481 | `fneg h0, h0` | ACCEPT → 0x1e214000 | ACCEPT → 0x1ee14000 | ACCEPT → 0x1ee14000 | differential |
| 482 | `fneg s0, d0` | ACCEPT → 0x1e214000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 483 | `fsqrt s0, s0, s0` | ACCEPT → 0x1e21c000 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 484 | `fsqrt h0, h0` | ACCEPT → 0x1e21c000 | ACCEPT → 0x1ee1c000 | ACCEPT → 0x1ee1c000 | differential |
| 485 | `fsqrt s0, d0` | ACCEPT → 0x1e21c000 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 486 | `dup v0.8b, w0, w0` | ACCEPT → 0x0e010c00 | ERR: unexpected characters following instruction at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 487 | `dup v0.8b, v0.b[16]` | ACCEPT → 0x0e010400 | ERR: register element index out of range 0 to 15 at operand 2 | ERR: vector lane must be an integer in range [0, 15]. | silent-accept |
| 488 | `dup v0.8b, v0.h[0]` | ACCEPT → 0x0e020400 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 489 | `dup v0.8b, x0` | ACCEPT → 0x0e010c00 | ERR: operand mismatch | ERR: invalid operand for instruction | silent-accept |
| 490 | `ldrb w0, [x0, x0, lsl #1]` | ACCEPT → 0x38607800 | ERR: invalid shift amount at operand 2 | ERR: expected 'lsl' or 'sxtx' with optional shift of #0 | silent-accept |
| 491 | `strb w0, [x0], x2` | ACCEPT → 0x39000000 | ERR: invalid addressing mode at operand 2 | ERR: index must be an integer in range [-256, 255]. | silent-accept |
| 492 | `ldrb d0, [x1, #0]` | ACCEPT → 0x3d400020 | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 493 | `ldrb w0, [x0, #-257]` | ACCEPT → 0x384ff000 | ERR: immediate offset out of range | ERR: index must be an integer in range [-256, 255]. | silent-accept |
| 494 | `ldrb sp, [x0]` | ACCEPT → 0x3d40001f | ERR: expected an integer or zero register at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 495 | `ldrb w0, [w0, #0]` | ACCEPT → 0x39400000 | ERR: expected a 64-bit base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 496 | `ldrb w0, [x0, w0]` | ACCEPT → 0x38604800 | ERR: invalid use of 32-bit register offset at operand 2 | ERR: expected 'uxtw' or 'sxtw' with optional shift of #0 | silent-accept |
| 497 | `ldr w0, [x0, #4]!` | ACCEPT → 0xb8404c00 | WARN: unpredictable transfer with writeback | ERR: unpredictable LDR instruction, writeback base is also a source | warning-class-CU |
| 498 | `strb w0, [xzr]` | ACCEPT → 0x390003e0 | ERR: invalid base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 499 | `ld2r {v0.8b, v1.8b}, [x1], #-1` | ACCEPT → 0x0ddfd020 | ERR: invalid post-increment amount at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 500 | `ld2r {v0.8b, v1.8b}, [x0], eq` | ACCEPT → 0x0d40d000 | ERR: writeback value must be an immediate constant at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 501 | `ld2r {v0.8b, v1.8b}, [d0]` | ACCEPT → 0x0d40d000 | ERR: invalid base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 502 | `ld2r {v0.8b, v1.8b}, [x1, #4]` | ACCEPT → 0x0d40d020 | ERR: invalid addressing mode at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 503 | `ld2r {v0.8b, v2.8b}, [x1]` | ACCEPT → 0x0d40d020 | ERR: the register list must have a stride of 1 at operand 1 | ERR: invalid operand for instruction | silent-accept |
| 504 | `ld2r {v0.8b, v1.8b}, [x1], x2` | ACCEPT → 0x0d40d020 | ACCEPT → 0x0de2c020 | ACCEPT → 0x0de2c020 | differential |
| 505 | `ld2r {v0.8b, v1.8b}, [x1]` | ACCEPT → 0x0d40d020 | ACCEPT → 0x0d60c020 | ACCEPT → 0x0d60c020 | differential |
| 506 | `ld2r {v0.8b, v1.8b}, [w0]` | ACCEPT → 0x0d40d000 | ERR: expected a 64-bit base register at operand 2 | ERR: invalid operand for instruction | silent-accept |
| 507 | `ld2r {v0.8b, v1.8b}, [xzr]` | ACCEPT → 0x0d40d3e0 | ERR: invalid base register at operand 2 | ERR: invalid operand for instruction | real — UPGRADED (user retest): word 0x0d40d3e0 is architecturally UNALLOCATED (objdump: .inst undefined; gcc valid [sp] form = 0x0d60c3e0) |
| 508 | (source-encoding) PUA U+E080..E0FF collision | see manual_tests.md | see manual_tests.md | — | real — witness test; gcc keeps bytes verbatim |
| 509 | (linker i686) -static: silent no-runtime when sysroot absent | see manual_tests.md | see manual_tests.md | — | real (conditional — repro requires absent /usr/i686-linux-gnu sysroot; filed as #509) |
| 510 | (frontend C) invalid float↔ptr casts bitcast | see manual_tests.md | see manual_tests.md | — | real — gcc rejects at compile; ccc accepts (issue #510) |

**TOTAL 509: real 506 (99.4%) · false positives 3 (#30 #119 #247 — user-confirmed: ccc AND gas both reject, only llvm-mc accepts; issues' "GNU alias" premise invalid on gas 2.42 → reclassify as llvm-mc-compat enhancement)**
