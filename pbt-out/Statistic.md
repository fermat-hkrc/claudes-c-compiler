# Statistic — FMBot-PBT 问题统计（全追踪器口径，自行验证）

**FMBot-PBT 发现了 2032 个功能类逻辑问题**（当前追踪器全部 open issue，API 逐页抓取）。经全量自行复核确认：**3 张假阳性**（#30/#119/#247）。已关闭的 35 张不计入。

**追踪器现况：open 2032 ＝ 有效缺陷 2029 ＋ 假阳性 3**（回填批 #2030-#2056 新增 27 张已验证单 + #2057/#2058 两张 HIGH-severity 猎获——全部经 oracle 逐张复证+全库查重净）；closed 35；总计 2067。

> **表口径（适用于一至四表）**：分类行合计 = **有效 2029**；其后单列**假阳性 3**（夹在两者之间）；**合计 = open 总数 2032**；**占比一律以 2032 为基数**。

> 分布说明：分类按"缺陷可观察后果"最大化迁移。剩余的双桶集中（运算 43.6%、功能缺失 40.1%）是本编译器的**真实缺陷结构**——一类是"非法输入被静默接受并编出错误指令"，一类是"合法输入被拒收"。

## 一、问题类型分布

| 问题类型 | 数量 | 占比（基数 2032） |
|---|---:|---:|
| 运算结果不正确（被接受的非法操作数/编码位错误→错误指令语义、错字节/错码/回绕/反转、C23 auto→int、_Complex 除法 NaN） | 891 | 43.9% |
| 读写出错的变量或内存（TLS 错址写穿、越界偏移回绕、重定位丢失/错型、错误寻址/栈帧/对齐） | 214 | 10.6% |
| 运行结果被丢弃（操作被静默归零或编为 NOP：#embed 零字节、输入未装载、依赖省略） | 35 | 1.7% |
| 编译器崩溃或卡死（ICE/panic、解析栈溢出、宏深度崩溃、负参数挂起） | 32 | 1.6% |
| 诊断信息缺失或有误（纯告警缺失、约束违反被接受、enum 自引用、旗标静默无效） | 28 | 1.4% |
| 程序运行时崩溃（编出的保留/非法字节 → SIGILL/#UD、GNU vector 运算 SIGSEGV） | 18 | 0.9% |
| 程序編譯不通過（合法輸入被拒收） | 811 | 39.9% |
| 　├ intrinsic 頭文件缺失（immintrin.h/arm_neon.h） | 249 | 12.3% |
| 　├ 指令表/偽指令缺失 | 133 | 6.5% |
| 　├ 內建函數派發缺失 | 96 | 4.7% |
| 　├ C23 語法/屬性/宏缺失 | 49 | 2.4% |
| 　└ 其他合法構造被拒收 | 284 | 14.0% |
| **有效小计** | **2029** | 99.85% |
| 假阳性（三方 oracle 齐拒亲测：#30/#119/#247） | 3 | 0.15% |
| **合计（open）** | **2032** | **100%** |

## 二、CWE 分类分布

| CWE | 名称（对应问题类型） | 数量 | 占比（基数 2032） |
|---|---|---:|---:|
| CWE-681 数值转换/运算不正确（含错误编码致错误指令语义） | 运算结果不正确 | 891 | 43.9% |
| CWE-125/CWE-787 越界读写（错址访问/错误重定位） | 读写出错的变量或内存 | 214 | 10.6% |
| CWE-229 值处理错误（结果丢失/归零） | 运行结果被丢弃 | 35 | 1.7% |
| CWE-674 不受控递归/CWE-476 空指针解引用 | 编译器崩溃或卡死 | 32 | 1.6% |
| CWE-754 异常条件检查不当/CWE-1078 输入验证不当 | 诊断信息缺失或有误 | 28 | 1.4% |
| CWE-241 不当数据处理导致崩溃 | 程序运行时崩溃 | 18 | 0.9% |
| N/A（程序編譯不通過；参考 CWE-691 控制流不完整） | 程序編譯不通過 | 811 | 39.9% |
| **有效小计** | | **2029** | 99.85% |
| 假阳性（N/A） | | 3 | 0.15% |
| **合计（open）** | | **2032** | **100%** |

## 三、严重程度（Severity）分布

| 严重程度 | 数量 | 占比（基数 2032） | 代表 issue |
|---|---:|---:|---|
| 高（HIGH：静默数据损坏/错误控制流；含 #2057 _Complex 除法 NaN） | 17 | 0.8% | #1692 TLS 初始化清零+写穿、#1698 asm goto 跳过语句、#1700 "x" 约束、#1702 auto→int、#1693 .so 重定位蒸发、#1628 fsub 反码、#1632 fcmp→NOP、#1490 mulx/pdep/pext 互换 |
| 中（MEDIUM：错误编码可致错行为、安全加固缺失、约束违反被接受） | 670 | 33.0% | #1695 RELRO 缺失、#1696 backtrace 单帧、#1710 #elifdef 走错分支、#1789 位域超宽接受、#2058 enum 自引用 |
| 低（LOW：纯功能缺失、罕用边角、告警类） | 1342 | 66.4% | 指令表/内建/intrinsic 缺失群（含 27 张回填）、#1712 __FILE_NAME__ |
| **有效小计** | **2029** | 99.85% | |
| 假阳性（原均为低阶语法类） | 3 | 0.15% | #30/#119/#247 BICS/EON/ORN 别名 |
| **合计（open）** | **2032** | **100%** | |

## 四、判定类别（Verdict-class）分布

按缺陷对 oracle 的表现形态划分（优先级：panic＞reverse＞differential＞silent-accept＞warn-accepts＞C-level＞missing/reject）：

| Verdict class（表现形态） | 数量 | 占比（基数 2032） | 说明 |
|---|---:|---:|---|
| missing/reject（合法输入被拒收/功能未实现：指令表、内建、intrinsic、C23；含 27 张回填） | 944 | 46.7% | ccc 拒收而 oracle 接受——零信息的功能缺失 |
| silent-accept（非法输入被静默接受并编码） | 900 | 44.3% | 参考工具链拒绝，ccc 零诊断编出机器码 |
| differential（编码/结果与参考工具链不同：错字节/NaN/回绕） | 110 | 5.4% | 双方皆接受但结果/字节级不同 |
| panic-on-invalid（非法输入触发编译器 panic/ICE/溢出） | 32 | 1.6% | 崩溃路径可达命令行 |
| C-level/linker/runtime（C 语言层、链接层、运行时行为） | 31 | 1.5% | TLS/ELF/执行时/内联汇编 |
| warn-accepts（参考仅告警或要求诊断，ccc 零输出；含旗标静默无效） | 10 | 0.5% | 告警缺失类 |
| reverse（操作数/顺序反转：显式 reversed/swapped） | 2 | 0.1% | #1628 fsub 反码、#1050 sgnj 反转 |
| **有效小计** | **2029** | 99.85% | |
| 假阳性（自行复核判定） | 3 | 0.15% | #30/#119/#247 |
| **合计（open）** | **2032** | **100%** | |

## 五、假阳性与重复验证明细（全部自行测试）

| 类别 | 数量 | 明细 |
|---|---:|---|
| 假阳性（三方 oracle 齐拒亲测） | 3 | #30 `bics x0,x1,#5`、#119 `eon`、#247 `orn`——gas/clang/ccc 齐拒，ccc 与参考一致 |

**全量假阳性扫描**：原始 2011 张全覆盖 + 新增单逐张复验 → 仅 3 张假阳性。**全量重复审计**：三阶段穷尽 + 新增单查重净 → 28 张重复已全部关闭。

**新增单复验**：全部经逐张独立 oracle 验证 + 全库查重净。

---
**方法说明**：母体 = 当前全部 open **2032** − 3 FP = **有效 2029**。四表分类行合计均为 2029（表一/二 891+214+35+32+28+18+811；表三 17+670+1342；表四 944+900+110+32+31+10+2），含 FP 合计均为 2032，占比基数均为 2032。

---

# Statistic — FMBot-PBT Issue Statistics (full-tracker scope, self-verified) — English Version

**FMBot-PBT discovered 2032 functional-logic issues** (all currently open issues, fetched via API). Full self-review confirmed: **3 false positives** (#30/#119/#247). 35 closed issues are excluded.

**Tracker status: open 2032 = 2029 valid + 3 false positives** (backfill #2030-#2056 added 27 validated issues + #2057/#2058 HIGH-severity finds — all individually oracle-verified and dedup-clean); closed 35; total 2067.

> **Table convention (Parts 1-4):** category rows sum to **valid 2029**; the **false-positive row (3)** sits between; **grand total = open 2032**; **all percentages use 2032 as base**.

## Part 1 — Problem-Type Distribution

| Problem type | Count | Pct (base 2032) |
|---|---:|---:|
| Incorrect computation results (invalid operands/encoding-bit errors → wrong semantics, wrong bytes/codes/wrap/reversal, C23 auto→int, _Complex div NaN) | 891 | 43.9% |
| Wrong variable/memory reads or writes (TLS wrong-address, offset wrap, lost relocations, bad addressing operands) | 214 | 10.6% |
| Runtime results dropped (operations zeroed or as NOP: #embed zero bytes, inputs never materialized) | 35 | 1.7% |
| Compiler crash or hang (ICE/panic, parser stack overflow, deep-macro crash) | 32 | 1.6% |
| Missing or wrong diagnostics (warning gaps, constraint violations accepted, enum self-reference, no-op flags) | 28 | 1.4% |
| Program crash at runtime (SIGILL/#UD, GNU vector SIGSEGV) | 18 | 0.9% |
| Compilation fails (valid input rejected) | 811 | 39.9% |
| 　├ Intrinsic headers missing | 249 | 12.3% |
| 　├ Instruction tables/directives missing | 133 | 6.5% |
| 　├ Builtin dispatch missing | 96 | 4.7% |
| 　├ C23 syntax/attributes/macros missing | 49 | 2.4% |
| 　└ Other valid constructs rejected | 284 | 14.0% |
| **Valid subtotal** | **2029** | 99.85% |
| False positives (triple-oracle rejection: #30/#119/#247) | 3 | 0.15% |
| **Total (open)** | **2032** | **100%** |

## Part 2 — CWE Distribution

| CWE | Name (mapped type) | Count | Pct (base 2032) |
|---|---|---:|---:|
| CWE-681 Incorrect numeric conversion/computation | Incorrect computation | 891 | 43.9% |
| CWE-125/CWE-787 Out-of-bounds read/write | Wrong memory access | 214 | 10.6% |
| CWE-229 Value handling error (lost/zeroed) | Results dropped | 35 | 1.7% |
| CWE-674/CWE-476 Uncontrolled recursion / NULL deref | Compiler crash/hang | 32 | 1.6% |
| CWE-754/CWE-1078 Improper check / input validation | Missing diagnostics | 28 | 1.4% |
| CWE-241 Improper data handling → crash | Runtime crash | 18 | 0.9% |
| N/A (compilation fails; cf. CWE-691) | Compilation fails | 811 | 39.9% |
| **Valid subtotal** | | **2029** | 99.85% |
| False positives (N/A) | | 3 | 0.15% |
| **Total (open)** | | **2032** | **100%** |

## Part 3 — Severity Distribution

| Severity | Count | Pct (base 2032) | Representative issues |
|---|---:|---:|---|
| High (silent data corruption / wrong control flow) | 17 | 0.8% | #1692 TLS init zeroed+cross-module, #1698 asm goto skip, #1700 "x"-constraint, #1702 auto→int, #1693 .so relocs vanish, #1628 fsub reversed, #1632 fcmp→NOP, #1490 mulx swap, #2057 _Complex div NaN |
| Medium (wrong encodings, missing hardening, constraint violations accepted) | 670 | 33.0% | #1695 no RELRO, #1696 backtrace 1-frame, #1710 #elifdef wrong branch, #1789 bit-field over-wide, #2058 enum self-ref |
| Low (pure feature gaps, obscure corners, warning-class) | 1342 | 66.4% | instruction/builtin/intrinsic families (incl. 27 backfill), #1712 __FILE_NAME__ |
| **Valid subtotal** | **2029** | 99.85% | |
| False positives (originally low-tier syntax) | 3 | 0.15% | #30/#119/#247 |
| **Total (open)** | **2032** | **100%** | |

## Part 4 — Verdict-Class Distribution

| Verdict class | Count | Pct (base 2032) | Notes |
|---|---:|---:|---|
| missing/reject (valid input rejected / unimplemented; incl. 27 backfill) | 944 | 46.7% | ccc rejects what the oracle accepts |
| silent-accept (invalid input silently accepted and encoded) | 900 | 44.3% | references reject; ccc emits with zero diagnostics |
| differential (encoding/result differs from reference) | 110 | 5.4% | both accept, but result/bytes differ |
| panic-on-invalid (invalid input triggers panic/ICE) | 32 | 1.6% | crash paths reachable from CLI |
| C-level/linker/runtime (C semantics, linker, runtime behavior) | 31 | 1.5% | TLS/ELF/execution/inline-asm |
| warn-accepts (reference warns; ccc silent) | 10 | 0.5% | warning-gap class |
| reverse (operand/order reversal) | 2 | 0.1% | #1628, #1050 |
| **Valid subtotal** | **2029** | 99.85% | |
| False positives (self-verified) | 3 | 0.15% | #30/#119/#247 |
| **Total (open)** | **2032** | **100%** | |

## Part 5 — False-Positive & Duplicate Verification Details

| Class | Count | Details |
|---|---:|---|
| False positives (triple-oracle rejection) | 3 | #30 `bics`, #119 `eon`, #247 `orn` — gas/clang/ccc all reject |

**Full FP sweep:** all issues covered, 3 false positives found. **Full dup audit:** three-stage exhausted, 28 duplicates all closed.

---
**Methodology:** base = all current open **2032** − 3 FP = **2029 valid**. All tables cross-consistent at 2029/2032.
