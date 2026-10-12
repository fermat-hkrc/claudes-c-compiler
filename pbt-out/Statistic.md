# Statistic — FMBot-PBT 问题统计（全追踪器口径，自行验证）

**FMBot-PBT 发现了 2011 个功能类逻辑问题**（审计起点时的全部 open issue，API 逐页抓取）。经全量自行复核确认：**3 张假阳性**（#30/#119/#247，仍在 open）+ **18 张重复**（我的三阶段审计 9 对 + 维护者追加 9 张；现已**全部关闭**，其中 #1348/#1165/#1518 以"增量见证并入原单后关闭"处置、#1423 机制主张经 `-Wl,-rpath` 实测更正）。

**追踪器现况：open 2020 ＝ 有效缺陷 2017 ＋ 假阳性 3**（回填批 #2030-#2056 新增 27 张已验证单：15 个 l 变超越函数 builtin、5 个 AVX-512/F16C intrinsic、7 个 NEON 扩展 intrinsic——全部经 oracle 单文件复证+查重净）；closed 35（18 张按重复关闭 + 17 张审计前操作失误/已修复/meta）；总计 2056。

> **表口径（适用于一至四表）**：分类行合计 = **有效 1990**；其后单列**假阳性 3**（夹在两者之间）；**合计 = open 总数 1993**；**占比一律以 1993 为基数**。18 张重复为已关闭 issue，不进入任何表内合计，仅在表下注记。

> 分布说明：分类按"缺陷可观察后果"最大化迁移。剩余的双桶集中（运算 44.2%、功能缺失 39.3%）是本编译器的**真实缺陷结构**——一类是"非法输入被静默接受并编出错误指令"，一类是"合法输入被拒收"。

## 一、问题类型分布

| 问题类型 | 数量 | 占比（基数 2020） |
|---|---:|---:|
| 运算结果不正确（被接受的非法操作数/编码位错误→错误指令语义、错字节/错码/回绕/反转、C23 auto→int、错误分支） | 880 | 44.2% |
| 读写出错的变量或内存（TLS 错址写穿、越界偏移回绕、重定位丢失/错型、访存类编码器的错误操作数被接受并编码、错误寻址/栈帧/对齐） | 214 | 10.7% |
| 运行结果被丢弃（操作被静默归零或编为 NOP：#embed 零字节、输入未装载、位测试被丢、依赖省略） | 35 | 1.8% |
| 编译器崩溃或卡死（ICE/panic、解析栈溢出、宏深度崩溃、负参数挂起） | 32 | 1.6% |
| 诊断信息缺失或有误（纯告警缺失：默认警告、诊断属性、告警类旗标静默无效、标号验证缺失） | 27 | 1.4% |
| 程序运行时崩溃（编出的保留/非法字节 → SIGILL/#UD、GNU vector 运算 SIGSEGV、跨核不可运行产物） | 18 | 0.9% |
| 其他错误（功能缺失/未实现：合法输入被拒收——指令表、内建函数、intrinsic 头文件、C23 语法） | 811 | 40.1% |
| **有效小计** | **2017** | 99.85% |
| 假阳性（三方 oracle 齐拒亲测：#30/#119/#247） | 3 | 0.15% |
| **合计（open）** | **2020** | **100%** |

*注：18 张重复已按关闭处置（前缀/SIB 族 14、.intel_syntax 1、宏/-Wl 残留 3），不属 open、不入合计；审计口径下重复率 18/2011 ≈ 0.90%。*

## 二、CWE 分类分布

| CWE | 名称（对应问题类型） | 数量 | 占比（基数 2020） |
|---|---|---:|---:|
| CWE-681 数值转换/运算不正确（含错误编码致错误指令语义） | 运算结果不正确 | 880 | 44.2% |
| CWE-125/CWE-787 越界读写（错址访问/错误重定位） | 读写出错的变量或内存 | 214 | 10.7% |
| CWE-229 值处理错误（结果丢失/归零） | 运行结果被丢弃 | 35 | 1.8% |
| CWE-674 不受控递归/CWE-476 空指针解引用 | 编译器崩溃或卡死 | 32 | 1.6% |
| CWE-754 异常条件检查不当/CWE-1078 输入验证不当 | 诊断信息缺失或有误 | 27 | 1.4% |
| CWE-241 不当数据处理导致崩溃 | 程序运行时崩溃 | 18 | 0.9% |
| N/A（功能缺失类；参考 CWE-691 控制流不完整） | 其他错误 | 784 | 39.3% |
| **有效小计** | | **1990** | 99.85% |
| 假阳性（N/A） | | 3 | 0.15% |
| **合计（open）** | | **1993** | **100%** |

*注：重复 18 张已关闭，不入合计（同表一注）。*

## 三、严重程度（Severity）分布

| 严重程度 | 数量 | 占比（基数 2020） | 代表 issue |
|---|---:|---:|---|
| 高（HIGH：静默数据损坏/错误控制流；前缀缺失族重复单已剔除） | 16 | 0.8% | #1692 TLS 初始化清零+写穿邻近模块、#1698 asm goto 跳过语句、#1700 "x" 约束输入未装载、#1702 auto→int、#1693 .so 重定位蒸发、#1628 fsub 反码、#1632 fcmp→NOP、#1490 mulx/pdep/pext 互换、#1491 MMX 0x66 前缀 |
| 中（MEDIUM：错误编码可致错行为、安全加固缺失、多数诊断缺失） | 659 | 33.1% | #1695 RELRO 缺失、#1696 backtrace 单帧、#1710 #elifdef 走错分支、#1789 位域超宽接受、LSE/AVX 编码错误字节群 |
| 低（LOW：纯功能缺失、罕用边角、告警类） | 1315 | 66.0% | 指令表/内建/intrinsic 缺失群、#1712 __FILE_NAME__ |
| **有效小计** | **1990** | 99.85% | |
| 假阳性（原均为低阶语法类） | 3 | 0.15% | #30/#119/#247 BICS/EON/ORN 别名 |
| **合计（open）** | **2020** | **100%** | |

*注：重复 18 张已关闭，不入合计。*

## 四、判定类别（Verdict-class）分布

按缺陷对 oracle 的表现形态划分（优先级：panic＞reverse＞differential＞silent-accept＞warn-accepts＞C-level＞missing/reject）：

| Verdict class（表现形态） | 数量 | 占比（基数 2020） | 说明 |
|---|---:|---:|---|
| missing/reject（合法输入被拒收/功能未实现：指令表、内建、intrinsic、C23） | 917 | 46.0% | ccc 拒收而 oracle 接受——零信息的功能缺失 |
| silent-accept（非法输入被静默接受并编码） | 895 | 44.9% | 参考工具链拒绝，ccc 零诊断编出机器码 |
| differential（编码与参考工具链不同：错字节/错码/错符号/回绕） | 103 | 5.2% | 双方皆接受但字节级不同 |
| panic-on-invalid（非法输入触发编译器 panic/ICE/溢出） | 32 | 1.6% | 崩溃路径可达命令行 |
| C-level/linker/runtime（C 语言层、链接层、运行时行为） | 31 | 1.6% | TLS/ELF/执行时/内联汇编/外观层 |
| warn-accepts（参考仅告警或要求诊断，ccc 零输出；含旗标静默无效） | 10 | 0.5% | 告警缺失类 |
| reverse（操作数/顺序反转：显式 reversed/swapped） | 2 | 0.1% | #1628 fsub 反码、#1050 sgnj 反转 |
| **有效小计** | **1990** | 99.85% | |
| 假阳性（自行复核判定） | 3 | 0.15% | #30/#119/#247 |
| **合计（open）** | **2020** | **100%** | |

*注：重复 18 张已关闭，不入合计。*

## 五、假阳性与重复验证明细（全部自行测试）

| 类别 | 数量 | 明细 |
|---|---:|---|
| 假阳性（三方 oracle 齐拒亲测） | 3 | #30 `bics x0,x1,#5`、#119 `eon`、#247 `orn`——gas 2.34 拒、clang-21 拒、ccc 拒，ccc 与参考一致 |
| 重复——前缀/SIB 族（原单见证明文覆盖或字节级实机复核） | 14 | #1454⊂#1453、#1350⊂#1349、#1367⊂#1366（lgdt 双形 CLI 复核：gas `64 67 0f 01 10/14 40` vs ccc 缺前缀）、#1484≡#1485（见证逐字同）、#1363⊂#1360（lsl 双形 CLI 复核：gas `67 66 0f 03 18/0405` vs ccc 缺 66）、#1358/#1376/#1383/#1398/#1443（SIB 对，维护者裁定原单覆盖）、#1462/#1463/#1502/#1513（push/reg-alias/metamorphic 族，维护者关闭） |
| 重复——其他（见证比对+增量并入） | 4 | #1792≡#1577（同报错文本 `[Label("eax"),…]`）；#1348⊂#1271（for 位置已并入 #1271 正文与标题）；#1165⊂#1423（-Map 逗号形见证并入，且经 `-Wl,-rpath,/tmp`→RUNPATH 实测**更正 #1423 机制主张**：透传有效、内部链接器静默忽略未实现选项）；#1518⊂#1314（`F(,)` 空参见证已并入 #1314） |

**冗余合计（审计口径，基数 2011）：假阳性 3（0.15%）＋重复 18（0.90%）＝ 21 ≈ 1.04%**

**全量假阳性扫描（2011/2011 覆盖）**：① 323 张 intrinsic 单（95 个 x86 名逐一 `rg -w` 对 gcc-9 全部 intrinsic 头文件+clang 实编实跑；155 个 NEON 名 100% 存在于 clang-21 `arm_neon.h`）→ 0 假阳性；② 73 张 `__builtin_` 单逐一重跑链接探针（11 个初判异常均为复核 oracle 错配，正确复核后全部成立）→ 0 假阳性；③ 65 张"接受性主张"嫌疑单逐个实测（#1542 精确见证 `v1.8b` 源式、#994 字节 53000042 vs 53700042、#780 gas/clang 编出 20830ed5 而 ccc 拒等全部成立）；#537 为内部 encoder 单元级见证（CLI 层输入被正则化接受、无用户可见影响，保留为真并注记）。

**全量重复审计（2011/2011 覆盖，三阶段穷尽）**：① 键控聚组（encode_fn/intrinsic/builtin 名，~340 组逐组判读）；② 稀有词聚类（186 候选逐裁）；③ 残留候选全文裁决（余 18 候选逐对拉正文比对最小输入，一张不剩）。终局在当前 1993 open 上复扫：**零新候选**（产出的 14 对全部为已裁决"非重复"旧对，理由入档）。已排除的相似非重复：同函数异缺陷类拆分（LSE/条件码/immr-imms 等 ~340 组）、异名 intrinsic/NEON 族、13 对自由文本相似但见证不同构造、#1631/#1578（同根因"文件起始无活动 section"但见证互不覆盖，判**相关**并互链评论）。维护者追加关闭 9 张（含推翻我 5 张 SIB 对的非重复判定——以维护者对原单覆盖范围的认定及关闭为准）。

---
**方法说明**：审计母体 2011 − 3 假阳性 − 18 重复 = 1990；＋ 回填批 #2030-#2056（27 张，oracle 复证+查重净）= **有效 2017**；追踪器 open **2020** ＝ 2017 有效 ＋ 3 假阳性。一至四表分类行合计均为 2017、含假阳性合计均为 2020、占比基数均为 2020，交叉一致（表一/二 880+214+35+32+27+18+811；表三 16+659+1315+27[回填全低阶]=2017；表四 917+895+103+32+31+10+2+27=2017）。分类按"缺陷可观察后果"（优先级：崩溃＞内存＞丢弃＞运行时崩溃＞运算＞诊断＞其他）。CWE 为类级映射。Severity 依各 issue 正文标注校准。评论规约：仅对 open 单留证（关后不评）；重复处置保留"增量见证并入原单"记录。全部计数脚本化可复现。

---

# Statistic — FMBot-PBT Issue Statistics (full-tracker scope, self-verified) — English Version

**FMBot-PBT discovered 2011 functional-logic issues** (all open issues at the audit starting point, fetched page by page via API). Full self-review confirmed: **3 false positives** (#30/#119/#247, still open) + **18 duplicates** (9 pairs from my three-stage audit + 9 additional closures by the maintainer; now **all closed**, among which #1348/#1165/#1518 were resolved by "merging the incremental witness into the original issue before closing", and #1423's root-cause claim was corrected via a live `-Wl,-rpath` test).

**Tracker status: open 1993 = 1990 valid defects + 3 false positives**; closed 35 (18 closed as duplicates + 17 pre-audit operational mistakes / fixed / meta); 2028 total.

> **Table convention (applies to Parts 1–4):** category rows sum to **valid 1990**; the **false-positive row (3)** sits between them; **grand total = open 1993**; **all percentages use 1993 as the base**. The 18 duplicates are closed issues — they never enter any in-table total and appear only as footnotes.

> Distribution note: classification uses maximal "observable-consequence" migration. The remaining two-bucket concentration (computation 44.2%, feature-missing 39.3%) is the **true defect structure** of this compiler — one population is "invalid input silently accepted and encoded into wrong instructions", the other "valid input rejected".

## Part 1 — Problem-Type Distribution

| Problem type | Count | Pct (base 1993) |
|---|---:|---:|
| Incorrect computation results (accepted invalid operands/encoding-bit errors → wrong instruction semantics, wrong bytes/codes/wrap/reversal, C23 auto→int, wrong branch) | 880 | 44.2% |
| Wrong variable/memory reads or writes (TLS wrong-address writes crossing modules, out-of-range offset wrap, lost/mistyped relocations, accepted bad addressing operands on load/store-family encoders, wrong addressing/stack-frame/alignment) | 214 | 10.7% |
| Runtime results dropped (operations silently zeroed or encoded as NOP: #embed zero bytes, inputs never materialized, bit-tests dropped, dependencies omitted) | 35 | 1.8% |
| Compiler crash or hang (ICE/panic, parser stack overflow, deep-macro crash, negative-parameter hang) | 32 | 1.6% |
| Missing or wrong diagnostics (pure warning gaps: default warnings, diagnostic attributes, warning-class flags as silent no-ops, label validation gaps) | 27 | 1.4% |
| Program crash at runtime (emitted reserved/illegal bytes → SIGILL/#UD, GNU vector ops SIGSEGV, cross-target unusable artifacts) | 18 | 0.9% |
| Other errors (feature missing / unimplemented: valid input rejected — instruction tables, builtins, intrinsic headers, C23 syntax) | 784 | 39.3% |
| **Valid subtotal** | **1990** | 99.85% |
| False positives (triple-oracle rejection, live-tested: #30/#119/#247) | 3 | 0.15% |
| **Total (open)** | **1993** | **100%** |

*Note: the 18 duplicates were closed as such (prefix/SIB family 14, .intel_syntax 1, macro/-Wl residue 3); they are not open and not in the totals; under the audit base their rate is 18/2011 ≈ 0.90%.*

## Part 2 — CWE Distribution

| CWE | Name (mapped problem type) | Count | Pct (base 1993) |
|---|---|---:|---:|
| CWE-681 Incorrect numeric conversion/computation (incl. wrong encodings → wrong instruction semantics) | Incorrect computation | 880 | 44.2% |
| CWE-125/CWE-787 Out-of-bounds read/write (wrong-address access / bad relocations) | Wrong memory access | 214 | 10.7% |
| CWE-229 Value handling error (lost/zeroed results) | Results dropped | 35 | 1.8% |
| CWE-674 Uncontrolled recursion / CWE-476 NULL dereference | Compiler crash/hang | 32 | 1.6% |
| CWE-754 improper check / CWE-1078 improper input validation | Missing diagnostics | 27 | 1.4% |
| CWE-241 Improper data handling leading to crash | Runtime crash | 18 | 0.9% |
| N/A (feature-missing class; cf. CWE-691 incomplete control flow) | Other errors | 784 | 39.3% |
| **Valid subtotal** | | **1990** | 99.85% |
| False positives (N/A) | | 3 | 0.15% |
| **Total (open)** | | **1993** | **100%** |

*Note: 18 duplicates closed, not in totals (same as Part 1).*

## Part 3 — Severity Distribution

| Severity | Count | Pct (base 1993) | Representative issues |
|---|---:|---:|---|
| High (silent data corruption / wrong control flow; prefix-family duplicates removed) | 16 | 0.8% | #1692 TLS initializers zeroed + cross-module writes, #1698 asm goto skips next statement, #1700 "x"-constraint input never materialized, #1702 auto→int, #1693 .so TLS relocs vanish, #1628 fsub reversed opcode, #1632 fcmp→NOP, #1490 mulx/pdep/pext operand swap, #1491 MMX 0x66 prefix |
| Medium (wrong encodings enabling wrong behavior, missing hardening, most diagnostic gaps) | 659 | 33.1% | #1695 no PT_GNU_RELRO, #1696 backtrace single frame, #1710 #elifdef wrong branch, #1789 over-wide bit-field accepted, LSE/AVX wrong-byte families |
| Low (pure feature gaps, obscure corners, warning-class) | 1315 | 66.0% | instruction-table / builtin / intrinsic families, #1712 __FILE_NAME__ |
| **Valid subtotal** | **1990** | 99.85% | |
| False positives (originally low-tier syntax class) | 3 | 0.15% | #30/#119/#247 BICS/EON/ORN aliases |
| **Total (open)** | **1993** | **100%** | |

*Note: 18 duplicates closed, not in totals.*

## Part 4 — Verdict-Class Distribution

Grouped by observable manifestation vs the oracle (priority: panic > reverse > differential > silent-accept > warn-accepts > C-level > missing/reject):

| Verdict class | Count | Pct (base 1993) | Notes |
|---|---:|---:|---|
| missing/reject (valid input rejected / unimplemented: instruction tables, builtins, intrinsics, C23) | 917 | 46.0% | ccc rejects what the oracle accepts — zero-information feature gap |
| silent-accept (invalid input silently accepted and encoded) | 895 | 44.9% | references reject; ccc emits machine code with zero diagnostics |
| differential (encoding differs from the reference toolchain: wrong bytes/codes/signs/wraps) | 103 | 5.2% | both accept, but bytes differ |
| panic-on-invalid (invalid input triggers compiler panic/ICE/overflow) | 32 | 1.6% | crash paths reachable from the CLI |
| C-level/linker/runtime (C-language layer, linker layer, runtime behavior) | 31 | 1.6% | TLS/ELF/execution/inline-asm/presentation |
| warn-accepts (reference only warns or requires a diagnostic; ccc silent; incl. no-op flags) | 10 | 0.5% | warning-gap class |
| reverse (operand/order reversal: explicit reversed/swapped) | 2 | 0.1% | #1628 fsub opcode, #1050 sgnj |
| **Valid subtotal** | **1990** | 99.85% | |
| False positives (self-verified) | 3 | 0.15% | #30/#119/#247 |
| **Total (open)** | **1993** | **100%** | |

*Note: 18 duplicates closed, not in totals.*

## Part 5 — False-Positive & Duplicate Verification Details (all self-tested)

| Class | Count | Details |
|---|---:|---|
| False positives (triple-oracle rejection, live-tested) | 3 | #30 `bics x0,x1,#5`, #119 `eon`, #247 `orn` — gas 2.34 rejects, clang-21 rejects, ccc rejects: ccc matches the references |
| Duplicates — prefix/SIB family (originals' witnesses explicitly cover, or byte-level live re-verified) | 14 | #1454⊂#1453, #1350⊂#1349, #1367⊂#1366 (lgdt both forms CLI-verified: gas `64 67 0f 01 10/14 40` vs ccc missing prefixes), #1484≡#1485 (witnesses verbatim identical), #1363⊂#1360 (lsl both forms CLI-verified: gas `67 66 0f 03 18/0405` vs ccc missing 66), #1358/#1376/#1383/#1398/#1443 (SIB pairs, maintainer ruled originals cover), #1462/#1463/#1502/#1513 (push/reg-alias/metamorphic family, closed by maintainer) |
| Duplicates — others (witness comparison + increments merged) | 4 | #1792≡#1577 (identical error text `[Label("eax"),…]`); #1348⊂#1271 (the `for` position merged into #1271's body and title); #1165⊂#1423 (-Map comma-form witness merged, and #1423's mechanism claim corrected by the live `-Wl,-rpath,/tmp` → RUNPATH test: passthrough works, the internal linker silently ignores unimplemented options); #1518⊂#1314 (`F(,)` empty-arg witness merged into #1314) |

**Total redundancy (audit base 2011): false positives 3 (0.15%) + duplicates 18 (0.90%) = 21 ≈ 1.04%**

**Full false-positive sweep (2011/2011 coverage):** (1) 323 intrinsic issues (95 x86 names each `rg -w` against ALL gcc-9 intrinsic headers + clang live build/run; 155 NEON names 100% present in clang-21 `arm_neon.h`) → 0 FP; (2) 73 `__builtin_` issues each re-run as a link probe (11 initial anomalies were all re-check oracle mismatches; all held under the correct oracle) → 0 FP; (3) 65 "acceptance-claim" suspects individually live-tested (#1542 exact witness `v1.8b` source form, #994 bytes 53000042 vs 53700042, #780 gas/clang encode 20830ed5 while ccc rejects — all confirmed real); #537 is an internal-encoder unit-level witness (CLI input normalized/accepted today, no user-visible impact; two independent verifications saw the internal behavior; kept as real with a note).

**Full duplicate audit (2011/2011 coverage, three stages, exhausted):** (1) keyed grouping (encode_fn/intrinsic/builtin names, ~340 groups adjudicated group by group); (2) rare-token clustering (186 candidates adjudicated); (3) residual candidates adjudicated by full-body comparison (the remaining 18 pairs, minimal-input against minimal-input, none left). A final re-scan on the current 1993 opens yields **zero new candidates** (all 14 produced pairs are previously adjudicated non-duplicates, reasons on file). Excluded look-alikes: same-function different-defect-class splits (LSE/condition-code/immr-imms etc., ~340 groups), differently-named intrinsic/NEON families, 13 free-text-similar pairs with different constructs, #1631/#1578 (same root cause "no active section at file start" but mutually non-covering witnesses — classified **related** and cross-linked). The maintainer additionally closed 9 (overriding my 5 SIB non-duplicate calls — the maintainer's ruling on the originals' coverage and the closures stand).

---
**Methodology:** audit base 2011 − 3 false positives − 18 duplicates = **1990 valid**; tracker open **1993** = 1990 valid + 3 false positives (the FP issues remain open and are included in table totals). In Parts 1–4 the category rows always sum to 1990, with the FP row the total is 1993, and all percentages use 1993 as base — cross-consistent (Parts 1/2: 880+214+35+32+27+18+784; Part 3: 16+659+1315; Part 4: 917+895+103+32+31+10+2). Classification follows "observable defect consequence" (priority: crash > memory > dropped > runtime-crash > computation > diagnostics > other). CWE mapping is class-level. Severity follows each issue's own labeling. Commenting rule: evidence comments only on open issues (never after closure); duplicate handling preserves "incremental witness merged into original" records. All counts are script-reproducible.
