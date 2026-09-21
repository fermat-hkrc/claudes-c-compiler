# ccc Issue-Tracker Verification Report — Full Run (all 509 issues)

*English version below; 中文版附于文末 (Chinese version at the end of this file).*

**Repository:** `fermat-hkrc/claudes-c-compiler` (branch `explore/pbt-test`, commit `bc6f5170`)
**Tracker scope:** all 509 open issues (closed #1 meta-stub and #511 duplicate excluded)
**Verification method:** Contract-Based Differential Validation (LLM agent) — every issue tested live, no sampling or extrapolation
**References:** gcc 13.3 (`aarch64-linux-gnu-gcc` / gas 2.42, warnings included), clang / llvm-mc (secondary), `qemu-i386` (i686 runtime), host gcc + qemu (x86-64 C level)
**Verification date:** 2026-09-15 · Artifacts: `pbt-out/verified_bug/` (38 reports), `pbt-out/full_verification_tracker.md` (509-row cross-check), `pbt-out/manual_tests.md`, `pbt-out/sampling/classification_per_issue.tsv` (per-issue category assignment)

---

## 1. Executive Summary

| Headline | Result |
|---|---|
| Coverage | **509/509 issues verified live** (503 ARM issues via the ccc-arm / gcc / clang command-line pipeline with `objdump` encoding comparison; 6 C-level and encoding issues verified individually) |
| **Defect realness** | **506 real (99.4%) · 3 false positives (0.6%, user-confirmed)** |
| Issue-text fidelity | 3 wording/severity amendments (#17, #150, #497); defects stand |
| Dominant compiler failure | Severity class S2 "invalid input silently accepted and encoded": 459/509 (90.2%) — invalid input is turned into machine code with zero diagnostics |
| Most harmful class | Severity class S1 "silent wrong code on valid input" (20 issues) — including the half-precision (`__fp16`) encoding family |
| Crash exposure | **15 crash-capable paths**: 9 panics reachable through the command-line interface + 6 latent encoder panics that an earlier parser stage or warning path hides |
| Fix leverage | A single trailing-operand check eliminates 80 defects; a shared register-slot checker addresses the 292-defect register family (C2–C4); one line fixes the 9-issue verified half-precision family |

**Bottom line:** the FM-Agent (property-based testing campaign) issue tracker is highly trustworthy — 506 of 509
filed reports correspond to reproducible defects cross-confirmed against the GNU and LLVM toolchains
(the 3 exceptions are alias-syntax enhancement requests valid only for llvm-mc, user-confirmed). The defining weakness of ccc
is not crashing but **diagnostic silence**: 479 of 506 real defects (94.7%) emit machine code with zero diagnostics —
459 (S2) encode invalid input that a reference toolchain rejects; 20 (S1) encode valid input wrongly where the references
encode it correctly.

---

## 2. Verification Methodology

Each issue's stated **Law** (contract) was compiled into a Hoare-triple probe
`{malformed input} encode {Err / expected-word}` and evaluated dynamically:

1. **Unit witness** — direct call to the encoder function under test (scratch test, kept as regression test)
2. **Reference arbitration** — the same assembly input assembled by gcc/gas **and** clang/llvm-mc;
   acceptance, error text, warnings, and emitted encodings all recorded
3. **Differential decode** — for accepted inputs, encodings extracted via `objdump` and compared word-for-word
4. **CONSTRAINED UNPREDICTABLE adjudication** — for architecturally CONSTRAINED UNPREDICTABLE constructs, both references'
   stances checked (gas accepts with a warning; llvm-mc rejects) before classifying

Every verdict is backed by a reproducible witness; nothing is judged by inspection alone.

---

## 3. Full-Tracker Statistics (n = 509)

### 3.1 Classification by ccc subsystem (where in the compiler the defect lives)

Taxonomy is **compiler-internal**: each real defect is attributed to the ccc layer whose missing
check or wrong bit-packing produces it (paths under `src/`). Counts are **re-derived from the 509-row
tracker by deterministic rules** (gcc/gas diagnostic text + input features + verdict class); the complete
per-issue assignment is in `classification_per_issue.tsv`, so every count below is traceable to individual
rows. These 8 categories supersede the earlier 10-layer (L1–L10) aggregate table; both taxonomies total 506.
CWE labels are dropped — see 3.2.

| Category | Missing / wrong mechanism in ccc | ccc code location | Real defects | % of 509 |
|---|---|---|---|---|
| C1 Operand-arity and addressing-mode acceptance | trailing extra operands, invalid register lists, invalid writeback/post-index forms silently consumed | per-mnemonic handlers, `src/backend/arm/assembler/encoder/*.rs`, `encoder/load_store.rs` | 108 | 21.2% |
| C2 Register-kind validation | floating-point/SIMD register accepted in an integer-register slot (or the reverse) | `encoder/data_processing.rs` and others | 91 | 17.9% |
| C3 Register-width and vector-arrangement consistency | 32/64-bit (W/X) widths mixed; NEON arrangement mismatch left unvalidated | register-field packing across encoders | 116 | 22.8% |
| C4 Stack-pointer / zero-register identity in encoding slot 31 | `sp`/`wsp`/`xzr`/`wzr` accepted where the other is required (bit 31 mis-encoded) | register-field packing across encoders | 85 | 16.7% |
| C5 Immediate, condition-code, and encoding-form selection validation | out-of-range immediates masked instead of rejected; AL/NV condition codes accepted; wrong encoding form selected (incl. the half-precision family) | immediate packing, `encoder/fp_scalar.rs` | 51 | 10.0% |
| C6 Shift/extend operator validation | wrong operator kind (`ror`, `lsr`, `lslx`) or amount beyond the register width accepted | shift-operand decoding | 35 | 6.9% |
| C7 Parser grammar and diagnostics | relocation modifiers accepted wrongly, valid branch/label syntax rejected, CONSTRAINED UNPREDICTABLE warnings absent | `src/backend/arm/assembler/parser.rs`, diagnostics layer | 14 | 2.8% |
| C8 Non-assembler subsystems | C frontend 4 (#2 #3 #4 #510) · source decoding 1 (#508) · linker/driver 1 (#509) | `src/backend/cast.rs` + `f128_softfloat.rs`, frontend lexer, `src/driver` | 6 | 1.2% |
| **Subtotal (real defects)** | | | **506** | **99.4%** |
| False positive | **no defect** — alias syntax accepted only by llvm-mc (#30 #119 #247); ccc matches gas 2.42, which also rejects | — | **3** | **0.6%** |
| **Total** | | | **509** | **100%**† |

† percentages rounded to 0.1; the column sums to 100.1% due to rounding.

One deterministic classifier, each of the 506 real issues counted exactly once. Under these rules the
6 missing-warning issues are counted in C7 (diagnostics); the 5 warn-then-accept issues fall into C1/C2/C3
by their operand-level defect. A category × severity cross-check reproduces the 3.2 ladder exactly (20/459/10/12/5).

**Reconciliation with the earlier aggregates (why 292, not 207):** the earlier taxonomy split register validation into three layers — L2 register class/width (207) + L3 arrangement (35) + L4 slot-31 (82) = **324 rows**; 207 was one slice of the family, never its total. The re-derivation draws the boundary by *the missing check*: every row whose missing check is a register-slot, width, arrangement, or slot-31 check lands in C2–C4 (292); rows in which a register merely appears but the missing check lies elsewhere leave the family — the 13 encoding-form-selection differentials sit in C5 (root cause the `fp_scalar.rs` type-field selection, not a register-slot check), extend-on-memory-operand checks in C6, writeback/addressing/operand-form checks in C1, missing CONSTRAINED UNPREDICTABLE warnings in C7. The earlier session published only layer aggregates, not per-issue assignments, so a row-by-row bridge to 207/35/82 cannot be reconstructed; the new assignment of all 506 real issues is fully published in `classification_per_issue.tsv`. Both taxonomies total 506.

Half-precision note: of the 13 encoding-form differentials in C5, 10 inputs use half-precision registers
(#355 #358 #361 #366 #409 #414 #475 #478 #481 #484); the 9-issue subset
(#358 #361 #366 #409 #414 #475 #478 #481 #484) was verified to share the single `fp_scalar.rs` root cause.

### 3.2 ccc severity ladder — by what the compiler ships (replaces the CWE roll-up)

Severity is defined by **compiler outcome**, not vulnerability taxonomy: what does ccc hand the
user for this input? Ranked worst-first:

| Severity | Compiler outcome (ccc behavior) | Definition | Issues | % of 509 |
|---|---|---|---|---|
| **S1** | **Silent wrong code on valid input** | valid source → wrong machine word, no diagnostic (17 ARM differential-encoding issues #15 #139 #264 #344 #354 #355 #358 #361 #366 #409 #414 #475 #478 #481 #484 #504 #505, plus C-level #2 #4 #508) | 20 | 3.9% |
| **S2** | **Invalid input silently accepted and encoded** | invalid input silently turned into machine code — composed of 453 silent acceptances (3.3, including #507 whose emitted word is architecturally unallocated), 4 parser-masked encoder acceptances proven at unit level but hidden behind earlier command-line parser rejections (#37 #52 #196 #198), and C-level #3 #510; full reconciliation in the cross-tabulation under 3.3 | 459 | 90.2% |
| **S3** | **Crash on malformed input** | panic; no wrong code shipped (9 panics reachable through the command-line interface #136 #234 #291 #379 #384 #436 #441 #456 #466, + #237 proven by unit test but hidden at the command-line level because the parser rejects first) | 10 | 2.0% |
| **S4** | **Missing or insufficient diagnostics** | encodes with a missing or wrong warning where the references diagnose (6 missing-warning CONSTRAINED UNPREDICTABLE issues #150 #163 #265 #320 #332 #497 + 5 issues where ccc warns but still accepts what the references reject #74 #80 #130 #131 #132, + #509 silent missing runtime) | 12 | 2.4% |
| **S5** | **Valid input rejected** | no wrong code; availability/compatibility loss (#36 #51 #55 #326 #411) | 5 | 1.0% |
| — | false positive (user-confirmed) | no defect; llvm-mc compatibility enhancement requests (#30 #119 #247) | 3 | 0.6% |
| **Total** | | | **509** | **100%**† |

† percentages rounded to 0.1; the column sums to 100.1% due to rounding.

Crash-family footnote: 5 of the S4 warn-but-still-accept rows also carry issue-reported, unit-test-proven latent
encoder panics (#74 #80 #130 #131 #132), so **15 crash-capable paths** exist in total (10 in S3 + 5 latent in S4).

Why not CWE: under a CWE roll-up, C1–C4 (400 issues, 78.6% of the tracker) collapse into the two generic
buckets CWE-20 "Improper Input Validation" / CWE-628, C5 into CWE-190/681, C6 into CWE-478 — one bucket absorbs
most of the tracker, all outcome information is lost (a CWE-20 row may crash, silently miscompile, or merely
miss a warning), and the "vulnerability" framing misstates the risk: this is a **correctness and robustness
failure of a compiler**, whose worst mode is shipping wrong bytes quietly.

### 3.3 Verdict-Class Distribution (raw behavior counts, all 509)

Verdict labels are the ones used in the `Verdict` column of `full_verification_tracker.md`. Note that 3.2 and 3.3 count on **different axes** and do not share row boundaries: 3.3 counts raw verdicts, while 3.2 groups verdicts by what the compiler ships — one severity class can merge several verdict classes (S2, S4), and one verdict class can split across severity classes (parser-masked, C-level specials). The cross-tabulation below reconciles the two tables cell by cell.

| Verdict class | Count | % of 509 |
|---|---|---|
| silent-accept (gcc rejects, ccc accepts) — including #507, whose accepted word is architecturally unallocated | 453 | 89.0% |
| differential (valid input, wrong encoding vs both references — including x86 #264 FS-segment prefix and the half-precision floating-point encoding family) | 17 | 3.3% |
| panic-on-invalid (panic reachable through the command-line interface) | 9 | 1.8% |
| warning-class CU (gas warns, ccc silent; CONSTRAINED UNPREDICTABLE forms) | 6 | 1.2% |
| parser-masked (encoder defect proven at unit level; the command-line parser rejects the input earlier, so the defect is not reachable through the command line) | 5 | 1.0% |
| warn-accepts (ccc emits a warning but then accepts input the references reject) | 5 | 1.0% |
| reverse (valid input rejected) | 5 | 1.0% |
| C-level / encoding specials (all real: #2 #3 #4 #508 #509* #510) — *#509 conditional on an absent i686 sysroot | 6 | 1.2% |
| **not-a-defect / false positive (user-confirmed)** | **3** | **0.6%** |
| **Total** | **509** | **100%**‡ |

‡ percentages rounded to 0.1; the column sums to 100.1% due to rounding (exact: 453/509 = 89.0%, not 88.8% — the old figure was computed before #507 was counted into silent-accept).

Mapping to the severity ladder (3.2) — cross-tabulation (rows = verdict classes of 3.3, columns = severity classes of 3.2; row and column totals reproduce both tables exactly):

| Verdict class (3.3) | S1 silent wrong code | S2 invalid input silently encoded | S3 crash | S4 deficient diagnostics | S5 valid input rejected | not-a-defect | Row total |
|---|---|---|---|---|---|---|---|
| silent-accept | — | **453** (incl. #507) | — | — | — | — | **453** |
| differential | **17** | — | — | — | — | — | **17** |
| panic-on-invalid | — | — | **9** | — | — | — | **9** |
| reverse | — | — | — | — | **5** | — | **5** |
| warning-class CU | — | — | — | **6** | — | — | **6** |
| parser-masked | — | 4 (#37 #52 #196 #198 — encoder accepts invalid input, proven at unit level) | 1 (#237 — unit-test panic) | — | — | — | **5** |
| warn-accepts | — | — | — | **5** | — | — | **5** |
| C-level / encoding specials | 3 (#2 #4 #508) | 2 (#3 #510) | — | 1 (#509) | — | — | **6** |
| not-a-defect / false positive | — | — | — | — | — | **3** | **3** |
| **Column total (= severity ladder 3.2)** | **20** | **459** | **10** | **12** | **5** | **3** | **509** |

### 3.4 Tracker Accuracy (full verification — no extrapolation)

- **506/509 real (99.4%) · 3 false positives (0.6%)** — every issue tested live
- The 3 false positives (#30, #119, #247 — BICS/EON/ORN "GNU immediate alias") are
  **user-confirmed**: ccc *and* gas 2.42 both reject; only llvm-mc accepts. The issues'
  "GNU alias" premise is invalid → reclassify as llvm-mc compatibility enhancement requests.
- Fidelity: 3 amendments (#17 wrap-target detail, #150 gas-claim + severity, #497 severity) — defects stand.

---

## 4. Tool Dimension (discovery vs verification are different jobs)

| Tool | Role | Issues filed | Issues adjudicated (verified live) | Own filings confirmed real | Own false positives |
|---|---|---|---|---|---|
| **FM-Agent (Hoare-style reasoning over property-based testing campaigns)** | discovery | 507 | — (not a verification tool) | 504 | 3 (#30 #119 #247) |
| **Contract-Based Differential Validation (LLM agent, this session)** | verification (+ side discovery) | 2 (#509 #510) | **509 (all)** | 2 | 0 |
| **Total** | | **509** | | **506** | **3** |

- The 3 false positives were *found by* the verification session but are *attributed to*
  FM-Agent's filings (single pattern: alias syntax accepted only by llvm-mc, which gas 2.42 also
  rejects — reference ambiguity, not detection hallucination).
- Beyond verdicts, the session produced 3 text amendments (#17, #150, #497) and
  2 reclassifications (#150 → missing-warning class, #509 → conditional on absent i686 sysroot).

The two tools are complementary: FM-Agent generates candidate defects; Contract-Based Differential Validation adjudicates
them against the GNU and LLVM reference toolchains. Only 2 issues came from the verification session
because discovery was not its objective.

## 5. Key Findings & Recommendations

1. **The compiler's defining defect is diagnostic silence, not crashes.** Silent emission (severity classes S1 + S2)
   = 479/506 real defects (94.7%): the 459 S2 defects encode invalid input that gcc/clang reject; the 20 S1 defects
   encode valid input wrongly where the references encode it correctly — in both cases bytes leave the compiler with
   zero diagnostics.
   The crash figure in this bullet is **contrast, not an addend**: only 15 of 506 defects can crash the compiler at all
   (10 in S3 + 5 latent panic paths inside S4; see the crash footnote in 3.2). That 15 is a cross-cutting count overlapping
   S4, so it is never added to 479. Full partition of the 506 real defects:
   **479 silent emission (S1+S2) + 15 crash-capable (all of S3 + 5 of S4) + 12 remainder (the other 7 S4
   diagnostics-only defects + the 5 S5 valid-input-rejected defects) = 506.**
2. **C1–C6 (486/506, 96.0%) are one disease: the encoder trusts its operands.** A single trailing-operand
   (arity) check eliminates 80 defects; a shared register-slot checker covers the 292-defect register
   family (C2–C4); range checks before every masked immediate address the remaining C5 silent/panic rows.
3. **The half-precision (`__fp16`) encoding family is the most user-harmful S1 class**: of the 13 encoding-form
   differentials in C5, 10 inputs use half-precision registers; the 9-issue verified subset
   (#358 #361 #366 #409 #414 #475 #478 #481 #484) shares a single root cause in
   `src/backend/arm/assembler/encoder/fp_scalar.rs` (the floating-point-type field selection checks only whether
   the register name starts with 'd', so half-precision registers fall through to the single-precision encoding) —
   wrong numerics, zero diagnostics.
4. **CONSTRAINED UNPREDICTABLE handling divergence (#150 #163 #320 #332 #497)**: ccc matches gas's acceptance but not its
   warnings; llvm-mc rejects. Recommend adopting gas-style warnings.

**Recommended actions (priority order):**
1. Fix the half-precision floating-point type-field selection in `encoder/fp_scalar.rs` (closes the 9-issue verified S1 family)
2. Introduce `get_gpr_checked` + `check_arity`: the arity check mechanically eliminates 80 trailing-extra-operand defects; the register-slot checker addresses the largest share of the 292-defect register family (C2–C4)
3. Add range checks before every bit-masked immediate — masking without prior validation is the C5 silent-wrap / panic family
4. Adopt gas-style warnings for the CONSTRAINED UNPREDICTABLE classes (STXP with status register also used as source; LDR with writeback where the transfer register equals the base register) for parity with both references
5. Reclassify #30 #119 #247 in the tracker as llvm-mc compatibility enhancement requests, not defects

---

*Report generated 2026-09-15 (full-run revision). All 38 verification reports: `pbt-out/verified_bug/`.
Full 509-row cross-check: `pbt-out/full_verification_tracker.md`. Per-issue category assignment:
`pbt-out/sampling/classification_per_issue.tsv`.*

---
---

# ccc 问题追踪验证报告 — 全量运行（全部 509 条问题）【中文版】

**仓库：** `fermat-hkrc/claudes-c-compiler`（分支 `explore/pbt-test`，提交 `bc6f5170`）
**追踪范围：** 全部 509 条开放问题（已关闭的 #1 元问题与 #511 重复项除外）
**验证方法：** 基于契约的差分验证（LLM agent）——每条问题均实测，无抽样、无外推
**参考工具：** gcc 13.3（`aarch64-linux-gnu-gcc` / gas 2.42，含告警）、clang / llvm-mc（次要）、`qemu-i386`（i686 运行时）、宿主 gcc + qemu（x86-64 C 层）
**验证日期：** 2026-09-15 · 产物：`pbt-out/verified_bug/`（38 份报告）、`pbt-out/full_verification_tracker.md`（509 行逐条交叉核对）、`pbt-out/manual_tests.md`、`pbt-out/sampling/classification_per_issue.tsv`（逐条类别分配）

---

## 1. 执行摘要

| 要点 | 结果 |
|---|---|
| 覆盖 | **509/509 条问题全部实测**（503 条 ARM 问题经 ccc-arm / gcc / clang 命令行管线并逐字比较 `objdump` 编码；6 条 C 层/编码问题逐条单独验证） |
| **缺陷真实性** | **506 条真实（99.4%）· 3 条误报（0.6%，经用户确认）** |
| 问题文本保真度 | 3 处措辞/严重度修正（#17、#150、#497）；缺陷本身成立 |
| 主导性编译器失效 | 严重度等级 S2"非法输入被静默接受并编码"：459/509（90.2%）——非法输入在零诊断的情况下被转成机器码 |
| 危害最大的类别 | 严重度等级 S1"合法输入被静默错误编码"（20 条）——含半精度（`__fp16`）编码家族 |
| 崩溃暴露面 | **15 条可崩溃路径**：9 条可经命令行界面触发的 panic + 6 条被更早的解析阶段或告警路径遮蔽的潜在编码器 panic |
| 修复杠杆 | 单一"尾随多余操作数"检查即可消除 80 条缺陷；共享的寄存器槽校验器可覆盖 292 条寄存器族缺陷（C2–C4）；一行代码修复经核实的 9 条半精度家族 |

**结论：** FM-Agent（性质测试活动）的问题追踪器高度可信——509 条报告中 506 条对应可复现、并经 GNU 与 LLVM 工具链交叉证实的缺陷（3 条例外是仅 llvm-mc 接受的别名语法增强请求，已经用户确认）。ccc 的定义性弱点不是崩溃，而是**诊断静默**：506 条真实缺陷中 479 条（94.7%）在零诊断下产出机器码——459 条（S2）编码了参考工具链会拒绝的非法输入；20 条（S1）把合法输入编码错误，而参考工具能正确编码。

---

## 2. 验证方法

每条问题所述的**定律**（契约）被编成 Hoare 三元组探针
`{malformed input} encode {Err / expected-word}` 并动态求值：

1. **单元见证** —— 直接调用被测编码器函数（临时测试，保留为回归测试）
2. **参考仲裁** —— 同一汇编输入分别由 gcc/gas **和** clang/llvm-mc 汇编；接受/拒绝、报错文本、告警、编码全部记录
3. **差分解码** —— 对被接受的输入，经 `objdump` 提取编码并逐字比较
4. **CONSTRAINED UNPREDICTABLE 裁定** —— 对架构上"受约束不可预测"的构造，先核对两个参考的立场（gas 接受并告警；llvm-mc 拒绝）再分类

每个裁定都有可复现见证；没有一条仅凭 inspection 判定。

---

## 3. 全量统计（n = 509）

### 3.1 按 ccc 子系统分类（缺陷位于编译器何处）

分类为**编译器内部视角**：每条真实缺陷归入"缺失的检查或错误的位打包"所在的 ccc 层（`src/` 下的路径）。计数由**确定性规则从 509 行 tracker 重新推导**（gcc/gas 诊断文本 + 输入特征 + 判定类别）；逐条分配完整保存在 `classification_per_issue.tsv`，下表每个数字都可回溯到具体行。这 8 类取代先前 L1–L10 聚合表；两种分类合计均为 506。CWE 标签弃用——见 3.2。

| 类别 | ccc 中缺失/出错的机制 | ccc 代码位置 | 真实缺陷 | 占 509 |
|---|---|---|---|---|
| C1 操作数个数与寻址模式检查缺失 | 尾随多余操作数、非法寄存器列表、非法写回/后递增形式被静默消费 | 各助记符处理，`src/backend/arm/assembler/encoder/*.rs`、`encoder/load_store.rs` | 108 | 21.2% |
| C2 寄存器种类校验 | 浮点/SIMD 寄存器被接受于整数寄存器槽（或反向） | `encoder/data_processing.rs` 等 | 91 | 17.9% |
| C3 寄存器宽度与向量 arrangement 一致性 | 32/64 位（W/X）宽度混用；NEON arrangement 不匹配未校验 | 各编码器寄存器字段打包 | 116 | 22.8% |
| C4 编码槽 31 的栈指针/零寄存器身份 | 要求另一者时接受了 `sp`/`wsp`/`xzr`/`wzr`（bit 31 编码错误） | 各编码器寄存器字段打包 | 85 | 16.7% |
| C5 立即数、条件码与编码形式选择校验 | 越界立即数被掩码而非拒绝；AL/NV 条件码被接受；编码形式选错（含半精度家族） | 立即数打包、`encoder/fp_scalar.rs` | 51 | 10.0% |
| C6 移位/扩展算子校验 | 错误算子类型（`ror`、`lsr`、`lslx`）或超出寄存器宽度的移位量被接受 | 移位操作数解码 | 35 | 6.9% |
| C7 解析器语法与诊断 | 重定位修饰符被错误接受、合法分支/标号语法被拒、CONSTRAINED UNPREDICTABLE 告警缺失 | `src/backend/arm/assembler/parser.rs`、诊断层 | 14 | 2.8% |
| C8 非汇编子系统 | C 前端 4（#2 #3 #4 #510）· 源码解码 1（#508）· 链接器/驱动 1（#509） | `src/backend/cast.rs` + `f128_softfloat.rs`、前端词法器、`src/driver` | 6 | 1.2% |
| **小计（真实缺陷）** | | | **506** | **99.4%** |
| 误报 | **无缺陷** —— 仅 llvm-mc 接受的别名语法（#30 #119 #247）；ccc 与同样拒绝它的 gas 2.42 行为一致 | — | **3** | **0.6%** |
| **合计** | | | **509** | **100%**† |

† 百分比四舍五入到 0.1；列合计因舍入为 100.1%。

单一确定性分类器，506 条真实缺陷每条恰好计一次。按此规则，6 条"缺告警"问题计入 C7（诊断）；5 条"告警后仍接受"问题按其操作数层缺陷落入 C1/C2/C3。类别×严重度交叉核对精确复现 3.2 的阶梯（20/459/10/12/5）。

**与旧聚合数的对账（为何是 292 而非 207）：**旧分类把寄存器校验拆成三层——L2 寄存器类/宽度（207）+ L3 arrangement（35）+ L4 槽 31（82）= **324 条**；207 只是其中一片，从来不是寄存器族的全貌。重新推导以"缺失的检查"划界：凡缺失的是寄存器槽、宽度、arrangement 或槽 31 检查的行，都落入 C2–C4（292）；寄存器只是出现在输入中、而真正缺失的是其他检查的行则移出族外——13 条编码形式选择 differential 归 C5（根因是 `fp_scalar.rs` 类型字段选择，并非寄存器槽检查）、内存操作数扩展检查归 C6、写回/寻址/操作数形式检查归 C1、缺失 CONSTRAINED UNPREDICTABLE 告警归 C7。旧会话只发布了各层聚合数、未发布逐条分配，故与 207/35/82 的逐行对照无法重建；新的 506 条逐条分配已完整发布于 `classification_per_issue.tsv`。两种分类合计均为 506。

半精度注记：C5 的 13 条编码形式 differential 中，10 条输入使用半精度寄存器（#355 #358 #361 #366 #409 #414 #475 #478 #481 #484）；其中 9 条（#358 #361 #366 #409 #414 #475 #478 #481 #484）经验证共享 `fp_scalar.rs` 单一根因。

### 3.2 ccc 严重度阶梯 —— 按编译器交付了什么（取代 CWE 汇总）

严重度按**编译器结果**而非漏洞分类学定义：对该输入，ccc 交给用户的是什么？按最坏优先排序：

| 严重度 | 编译器结果（ccc 行为） | 定义 | 数量 | 占 509 |
|---|---|---|---|---|
| **S1** | **合法输入被静默错误编码** | 合法源码 → 错误机器字、零诊断（17 条 ARM 差分编码 #15 #139 #264 #344 #354 #355 #358 #361 #366 #409 #414 #475 #478 #481 #484 #504 #505，加 C 层 #2 #4 #508） | 20 | 3.9% |
| **S2** | **非法输入被静默接受并编码** | 非法输入被静默转成机器码——由 453 条静默接受（见 3.3，含 #507 其产出的字在架构上未分配）、4 条单元级证实但被命令行解析器先行拒绝所遮蔽的编码器接受（#37 #52 #196 #198）、C 层 #3 #510 构成；完整对账见 3.3 交叉表 | 459 | 90.2% |
| **S3** | **恶意输入导致崩溃** | panic；不产出错误代码（9 条可经命令行触发的 panic #136 #234 #291 #379 #384 #436 #441 #456 #466，+ #237 经单元测试证实但因解析器先拒绝而在命令行层不可见） | 10 | 2.0% |
| **S4** | **诊断缺失或不足** | 在参考工具给出诊断之处以缺失/错误告警编码（6 条缺失告警的 CONSTRAINED UNPREDICTABLE #150 #163 #265 #320 #332 #497 + 5 条 ccc 告警后仍接受参考所拒输入 #74 #80 #130 #131 #132，+ #509 静默缺失运行时） | 12 | 2.4% |
| **S5** | **拒绝合法输入** | 不产出错误代码；可用性/兼容性损失（#36 #51 #55 #326 #411） | 5 | 1.0% |
| — | 误报（用户确认） | 无缺陷；llvm-mc 兼容性增强请求（#30 #119 #247） | 3 | 0.6% |
| **合计** | | | **509** | **100%**† |

† 百分比四舍五入到 0.1；列合计因舍入为 100.1%。

崩溃族脚注：S4 的 5 条"告警后仍接受"行还带有问题报告/单元测试证实的潜在编码器 panic（#74 #80 #130 #131 #132），故**可崩溃路径共 15 条**（S3 中 10 条 + S4 中潜在 5 条）。

为何不用 CWE：在 CWE 汇总下，C1–C4（400 条，占 78.6%）会塌缩进 CWE-20"输入校验不当"/CWE-628 两个泛化桶，C5 进 CWE-190/681，C6 进 CWE-478——一个桶吞掉大半追踪器，结果信息全部丢失（一条 CWE-20 可能崩溃、静默错误编译、或仅缺一条告警），且"漏洞"框架错述了风险：这是**编译器的正确性与健壮性失效**，其最坏模式是安静地交付错误字节。

### 3.3 判定类别分布（原始行为计数，全部 509）

判定标签与 `full_verification_tracker.md` 的 `Verdict` 列一致。注意 3.2 与 3.3 计数轴不同、行边界不重合：3.3 按原始判定计数，3.2 按编译器交付结果分组——一个严重度类别可合并多个判定类别（S2、S4），一个判定类别也可拆入多个严重度类别（parser-masked、C 层特例）。下方交叉表逐格对账。

| 判定类别 | 数量 | 占 509 |
|---|---|---|
| silent-accept（gcc 拒绝、ccc 接受）——含 #507（其接受的字在架构上未分配） | 453 | 89.0% |
| differential（合法输入、相对两个参考编码错误——含 x86 #264 FS 段前缀与半精度浮点编码家族） | 17 | 3.3% |
| panic-on-invalid（可经命令行触发的 panic） | 9 | 1.8% |
| warning-class CU（gas 告警、ccc 静默；CONSTRAINED UNPREDICTABLE 形式） | 6 | 1.2% |
| parser-masked（编码器缺陷经单元级证实；命令行解析器更早拒绝该输入，故缺陷不可经命令行触达） | 5 | 1.0% |
| warn-accepts（ccc 发出告警但仍接受参考工具拒绝的输入） | 5 | 1.0% |
| reverse（合法输入被拒绝） | 5 | 1.0% |
| C 层/编码特例（均真实：#2 #3 #4 #508 #509* #510）——*#509 以缺失 i686 sysroot 为条件 | 6 | 1.2% |
| **非缺陷/误报（用户确认）** | **3** | **0.6%** |
| **合计** | **509** | **100%**‡ |

‡ 百分比四舍五入到 0.1；列合计因舍入为 100.1%（精确值：453/509 = 89.0%，而非 88.8%——旧值是在 #507 计入 silent-accept 之前算出的）。

到严重度阶梯（3.2）的映射——交叉表（行 = 3.3 判定类别，列 = 3.2 严重度类别；行、列合计精确复现两表）：

| 判定类别（3.3） | S1 静默错误编码 | S2 非法输入被静默编码 | S3 崩溃 | S4 诊断不足 | S5 拒绝合法输入 | 非缺陷 | 行合计 |
|---|---|---|---|---|---|---|---|
| silent-accept | — | **453**（含 #507） | — | — | — | — | **453** |
| differential | **17** | — | — | — | — | — | **17** |
| panic-on-invalid | — | — | **9** | — | — | — | **9** |
| reverse | — | — | — | — | **5** | — | **5** |
| warning-class CU | — | — | — | **6** | — | — | **6** |
| parser-masked | — | 4（#37 #52 #196 #198——编码器接受非法输入，单元级证实） | 1（#237——单元测试 panic） | — | — | — | **5** |
| warn-accepts | — | — | — | **5** | — | — | **5** |
| C 层/编码特例 | 3（#2 #4 #508） | 2（#3 #510） | — | 1（#509） | — | — | **6** |
| 非缺陷/误报 | — | — | — | — | — | **3** | **3** |
| **列合计（= 严重度阶梯 3.2）** | **20** | **459** | **10** | **12** | **5** | **3** | **509** |

### 3.4 追踪器准确性（全量验证——无外推）

- **506/509 真实（99.4%）· 3 条误报（0.6%）**——每条问题均实测
- 3 条误报（#30、#119、#247——BICS/EON/ORN"GNU 立即数别名"）**已经用户确认**：ccc 与 gas 2.42 均拒绝，仅 llvm-mc 接受。问题所述"GNU 别名"前提不成立 → 重分类为 llvm-mc 兼容性增强请求。
- 保真度：3 处修正（#17 回绕目标细节、#150 gas 主张与严重度、#497 严重度）——缺陷本身成立。

---

## 4. 工具维度（发现与验证是不同的工作）

| 工具 | 角色 | 提交问题数 | 裁定数（实测） | 自身提交被证实真实 | 自身误报 |
|---|---|---|---|---|---|
| **FM-Agent（基于性质测试活动的 Hoare 式推理）** | 发现 | 507 | —（非验证工具） | 504 | 3（#30 #119 #247） |
| **基于契约的差分验证（LLM agent，本次会话）** | 验证（+ 顺带发现） | 2（#509 #510） | **509（全部）** | 2 | 0 |
| **合计** | | **509** | | **506** | **3** |

- 3 条误报由验证会话*发现*，但*归属*于 FM-Agent 的提交（单一模式：仅 llvm-mc 接受、gas 2.42 同样拒绝的别名语法——参考歧义，而非检测幻觉）。
- 除裁定外，本次会话产出 3 处文本修正（#17、#150、#497）与 2 处重分类（#150 → 缺告警类；#509 → 以缺失 i686 sysroot 为条件）。

两种工具互补：FM-Agent 生成候选缺陷；基于契约的差分验证对照 GNU 与 LLVM 参考工具链裁定之。仅 2 条问题来自验证会话，因为发现并非其目标。

## 5. 关键发现与建议

1. **编译器的定义性缺陷是诊断静默，不是崩溃。** 静默产出（S1 + S2）= 479/506 真实缺陷（94.7%）：459 条 S2 编码了 gcc/clang 拒绝的非法输入；20 条 S1 把合法输入编码错误而参考工具编码正确——两种情况下字节都带着零诊断离开编译器。
   本条中的崩溃数字是**对比，不是加数**：506 条缺陷中仅 15 条可令编译器崩溃（S3 中 10 条 + S4 内 5 条潜在 panic 路径；见 3.2 崩溃族脚注）。该 15 为横切计数、与 S4 重叠，故永不与 479 相加。506 条真实缺陷的完整三分：
   **479 静默产出（S1+S2）+ 15 可崩溃（S3 全部 + S4 中 5 条）+ 12 其余（S4 其余 7 条纯诊断缺陷 + S5 的 5 条拒绝合法输入）= 506。**
2. **C1–C6（486/506，96.0%）是同一种病：编码器无条件信任其操作数。** 单一"尾随操作数"（个数）检查消除 80 条；共享寄存器槽校验器覆盖 292 条寄存器族（C2–C4）；每个被掩码立即数前加范围检查可解决 C5 其余静默/panic 行。
3. **半精度（`__fp16`）编码家族是危害最大的 S1 类**：C5 的 13 条编码形式 differential 中 10 条输入使用半精度寄存器；经核实的 9 条子集（#358 #361 #366 #409 #414 #475 #478 #481 #484）共享 `src/backend/arm/assembler/encoder/fp_scalar.rs` 中单一根因（浮点类型字段选择只检查寄存器名是否以 'd' 开头，半精度寄存器因此落入单精度编码）——数值错误、零诊断。
4. **CONSTRAINED UNPREDICTABLE 处理分歧（#150 #163 #320 #332 #497）**：ccc 与 gas 的接受行为一致但不带其告警；llvm-mc 拒绝。建议采纳 gas 式告警。

**建议行动（按优先级）：**
1. 修复 `encoder/fp_scalar.rs` 中的半精度浮点类型字段选择（关闭经核实的 9 条 S1 家族）
2. 引入 `get_gpr_checked` + `check_arity`：个数检查机械消除 80 条尾随多余操作数缺陷；寄存器槽校验器覆盖 292 条寄存器族（C2–C4）的最大份额
3. 每个位掩码立即数前加范围检查——无先验校验的掩码即 C5 的静默回绕 / panic 族
4. 对 CONSTRAINED UNPREDICTABLE 类别采纳 gas 式告警（STXP 状态寄存器同时作源；LDR 写回且传输寄存器等于基址寄存器），与两个参考对齐
5. 在追踪器中将 #30 #119 #247 重分类为 llvm-mc 兼容性增强请求，而非缺陷

---

*报告生成于 2026-09-15（全量修订版）。38 份验证报告：`pbt-out/verified_bug/`。509 行完整交叉核对：`pbt-out/full_verification_tracker.md`。逐条类别分配：`pbt-out/sampling/classification_per_issue.tsv`。*
