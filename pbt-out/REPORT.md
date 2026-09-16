# PBT Campaign Report: claudes-c-compiler (ccc)

## Summary

**Date:** 2026-09-15
**Repository:** fermat-hkrc/claudes-c-compiler (branch `explore/pbt-test`)
**Modules tested:** e2e_diff (full compiler pipeline, x86-64), common::encoding, common::const_arith, common::types
**Tests:** 12 properties (8 E2E differential × 1000 cases + 1 strengthening round × 400 cases + 4 unit properties × 1000 cases) + 1 ignored deterministic witness (issue #508)
**Effort tier:** standard(约 30 分钟名义预算;实际因调试生成器与长时 E2E 运行超支,如实记录)
**Result:** 12 passed, 0 failed — plus 1 low-severity candidate finding (PUA collision, issue #508 filed)

性质测试结果:`Results: 12 passed, 0 failed`(第二轮扩展:浮点算术 P10、switch/goto/三元 P11 各 1000 例全过)

- 差分 oracle:E2E 性质将随机生成的无 UB、确定性 C 程序分别交给 `ccc`(x86-64)与 `gcc 13.3` 编译运行,比较 (stdout, exit code)。规范证据:DESIGN_DOC.md:164 "x86-64 code generation (SysV AMD64 ABI)"。
- 强化轮(P1s):叶节点仅取边界值(INT_MIN/MAX、UINT_MAX、LONG_MIN 等)、表达式深度 3→4、类型转换频率加倍 —— 400 例全过。
- 仓库既有 497 个 lib 测试全部通过(证明 harness 接入无破坏)。

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|--------------|
| e2e_diff (driver→lexer→parser→sema→IR→opt→codegen→asm→link) | 8 (P1-P5, P1s, P10-P11) | 0 confirmed / 1 candidate (PUA 见注) | differential vs gcc-13.3 |
| common::encoding | 2 (P6/P7) | 0 (1 candidate, 见 Bugs Found) | algebraic round_trip / idempotence |
| common::const_arith | 1 (P8) | 0 | differential vs independent C11 reference model |
| common::types | 1 (P9) | 0 | algebraic invariant (minimality + overflow branch) |

## Bugs Found

1 candidate(低危,待用户定夺是否开 issue —— 已按用户工作流先写报告、查重 gh issue 无重复):

**Bug: PUA 编码静默破坏含字面 U+E080..U+E0FF 字符的源文件**
- **Law:** `bytes_to_string` → lexer `decode_pua_byte` 必须保真还原任意被接受输入的字节(模块文档 encoding.rs:8-10)。
- **Minimal input:** `[0xEE, 0x82, 0x80]`(合法 UTF-8 的 U+E080)→ 编码直通 → 解码端 `decode_pua_byte` 命中 `EE 82 80` 模式 → 还原为单字节 `[0x80]`。3 字节静默变 1 字节。
- **Expected vs Actual:** 期望保真(或文档声明限制/拒绝);实际静默数据丢失。
- **Root cause:** PUA 转码方案固有信息碰撞 —— 方案保留区(U+E080..U+E0FF)与合法 UTF-8 输入空间重叠,编码端无法区分"我编码的"与"源文件原有的"。
- **Impact:** 含 PUA 字符的字符串字面量内容被静默改写,无诊断。现实概率低(gcc/clang 根本不转码,直接保字节)。
- **Severity:** low
- **Bug report:** `pbt-out/bug_reports/encoding_pua_collision.md`(含 Found by 段)
- **Regression test:** (尚未写;P6 现以碰撞前条件过滤,可在定夺后加确定性 witness)

## Design Caveats (if any)

- **函数实参求值顺序差异(观察,非 bug)**:多副作用实参 `printf("%d %d %d", bump(), bump(), bump())`:ccc 从左到右求值,gcc 从右到左。
  Spec evidence: C11 6.5.2.2p10 —— "There is no sequence point before or after the evaluation of function arguments; the order of evaluation of function arguments is unspecified." 两编译器均合规;性质生成器已规避对该顺序的依赖。
- **未初始化 union 字节残留(观察,非 bug)**:栈上 union 未初始化字节 ccc 倾向零、gcc 倾向栈垃圾。
  Spec evidence: C11 6.3.2.1p2(indeterminate value)/ 6.2.6.1 —— 读取未初始化自动存储对象的值是 indeterminate,无平台契约可比。

(PUA 碰撞无独立文档证据,已按 bug-by-default 归入 ## Bugs Found,不再在此重复。)

## Test Files Created

| File | Tests |
|------|-------|
| tests/e2e_diff.rs | 8 (e2e_arith_expr, e2e_arith_boundary, e2e_control_flow, e2e_struct_layout, e2e_calls_abi, e2e_globals_init, e2e_float_arith, e2e_control_flow_2) |
| src/common/pbt_tests/mod.rs | (linker) |
| src/common/pbt_tests/encoding_pbt.rs | 3 (pua_roundtrip, encode_idempotent, pua_collision_witness #[ignore]) |
| src/common/pbt_tests/const_arith_pbt.rs | 1 (const_binop_matches_c_semantics) |
| src/common/pbt_tests/types_pbt.rs | 1 (align_up_laws) |
| src/common/mod.rs | +2 行 `#[cfg(test)] mod pbt_tests;` |

## Output Directories

- `pbt-out/` — PLAN.md, PROPERTIES.md, FUNCTION_INDEX.md, COVERAGE.md, COVERAGE_STATUS.md, INVARIANTS.md, REPORT.md(本文件)
- `pbt-out/bug_reports/encoding_pua_collision.md` — 唯一候选 bug 报告
- `pbt-out/code-coverage/` — 本环境无覆盖率插桩(CARGO_RUSTFLAGS 为空、无 .profraw),数据驱动的 contract-surface sweep 无法执行,如实记录;强化轮以 oracle 强化替代(见 Summary)

## Sweep(标准档 1 轮)执行情况

`coverage_gaps` 工具返回 "No instrumented coverage data yet"(环境未注入覆盖率插桩)。替代动作:强化轮 P1s(边界偏斜 + 深度提升)已执行并通过;PUA 碰撞作为 P6 域外反例被人工三角验证(代码路径推导 + 既有单测交叉核对 `test_roundtrip_all_bytes`)。

---

## Post-campaign session: manual issue verification (2026-09-15, later)

Verifying tracker issues one-by-one (user reproduces manually, then confirmed):

| Issue | Function | Verdict |
|---|---|---|
| #2 | IrConst::cast_float_to_target (F128) | real — panic + silent miscompile (E2E vs gcc) |
| #3 | IrConst::cast_float_to_target (U8/U16) | real (latent) — contract violation, no E2E miscompile |
| #4 | classify_cast_with_f128 | real — unit + asm (fildl vs fildq) + qemu runtime; f2p mechanism shielded |
| #5 | encode_adc | real — trailing shift silently ignored (unit, failing assert) |
| #14 | encode_add_sub | real — ROR/oob-amount/extend-imm3/unknown-kind all silently accepted (unit) |

Side-findings filed: #509 (i686 -static entry=main), #510 (invalid float↔ptr casts bitcast).
Random sample of 100 drawn (seed=42) → `pbt-out/sampling/sample_100_tracker.md` — **completed: 100/100 verified, 100% real** (3 wording/severity amendments: #17 wrap detail, #150/#497 warning-class). **Followed by FULL verification of all 509 issues** (assembler-CLI pipeline ccc-arm/gcc/clang + 6 C-level individually): **506 real (99.4%) · 3 false positives** (#30/#119/#247, user-confirmed — llvm-mc-only aliases, gas-parity). Artifacts: `pbt-out/full_verification_tracker.md` (509-row cross-check), `pbt-out/manual_tests.md` (per-issue test commands), `pbt-out/sampling/all_issues_stats.md` (presentation report); per-issue evidence: 43 reports in `pbt-out/verified_bug/`; issue comments posted: #2, #4 (+#509, #510 filed, #511 dupe closed).

### Sweep round (this session, 1 round)

`coverage_gaps`: "No instrumented coverage data yet" (no llvm-profdata/cov, RUSTFLAGS empty, no
.profraw — same environmental limit as the main campaign). Substitute: code-reading-driven gap
analysis of the tests written this session. Found and covered: issue #14's extend-register arm
(documented in the issue's law: "imm3 > 4 is UNALLOCATED"; code: `_ => 0b011` default +
`imm3 = *amount & 0x7` mask) was untested by the first #14 probe. Added two targeted probes
(`sxtw #8` → silent truncation to imm3=0; unknown kind `foo` → silent UXTX default). Evidence:
`ror #0` and `lsl #64` encode to the identical word 0x0B020000 (amount masked & 0x3F). All
four invalid inputs return Ok — sweep strengthened the #14 witness from 2 to 4 failing probes.

## 备注

- 测试开发期暴露并修复的 6 个测试侧问题(非 SUT bug):i128 区间计算溢出 panic、int 传 `%lu` 的 UB、位域取地址非法 C、union 未初始化字节转储 UB、实参求值顺序依赖、程序返回值与 `timeout` 退出码 124 碰撞、数组字段越界索引 —— 全部为生成器/harness 缺陷,修复后 0 失败。这些调试经历本身即验证了差分 oracle 的灵敏度(gcc/ccc 行为分歧均被捕获并正确归类)。
- 已提交 commit `2f1c4ce7`(harness + 初版产物)推至 `origin/explore/pbt-test`;最终产物更新待用户审阅 bug 报告后一并提交。
