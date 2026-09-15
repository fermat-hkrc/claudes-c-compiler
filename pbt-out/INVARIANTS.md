# INVARIANTS — claudes-c-compiler (confirmed during 2026-09-15 PBT campaign)

供后续 campaign 的 Scan 阶段读取。

## Confirmed invariants (10 properties, all passing, 2026-09-15)

- E2E(x86-64,vs gcc 13.3):无 UB、确定性 C 程序在 ccc 与 gcc 下 (stdout, exit_code) 全等 —— 覆盖
  整数算术(含边界偏斜深度 4)、有界控制流、struct/union/位域布局观测、SysV 调用约定
  (0-10 标量参数、按值小结构体往返、MEMORY 类、有界递归、函数指针)、全局/静态常量初始化。
  (tests/e2e_diff.rs,5×1000 + 1×400 例)
- `bytes_to_string ∘ decode_pua_byte` 在非 BOM 前缀且不含 U+E080..U+E0FF 字面序列的输入上为恒等。
- `bytes_to_string` 幂等(非 BOM 前缀输入)。
- `eval_const_binop`(I64 路径)与独立 C11 参照模型在无 UB 域上逐例相等(含 32/64 位 × 有符号/
  无符号 × 18 运算 × 边界值池)。
- `align_up(x, a)`(a 为 2 的幂):r ≥ x、a | r、最小性;溢出分支按代码文档返回 x 原值。

## Environment quirks(下次 campaign 注意)

- gcc 与 ccc 实参求值顺序相反(均合规)—— 生成含多副作用实参的表达式会产生假阳性,必须先赋临时变量。
- `timeout` 退出码 124 与程序合法返回值 124 碰撞 —— 生成程序返回值须避开 124-127(本 campaign 用 % 100)。
- 源码中 `union`/`struct` 未初始化字节为 indeterminate —— 布局转储前必须完整初始化(如 `= {0}`)。
- 位域不可取地址 —— offsetof 探针只能用于普通字段。
- /mnt/c(9p)文件系统上编译慢 ~3 倍;E2E 用 /tmp 工作目录。
- bash 工具 60s 超时会连带杀死未完全脱离的后台进程 —— 用 `setsid bash -c '...' < /dev/null` 启动长测试。
- 环境未注入覆盖率插桩(CARGO_RUSTFLAGS 空、无 .profraw)—— coverage_gaps 工具不可用。
- proptest 会把失败种子持久化到 tests/*.proptest-regressions 并在下次运行重放 —— 生成器修复后
  旧种子会自动转绿,文件可安全入库。
- ccc 编译速度约为 gcc 的 2-3 倍(0.2s vs 0.58s,同一输入)。

## Known candidate finding(未定夺)

- PUA 碰撞:`pbt-out/bug_reports/encoding_pua_collision.md`(低危;U+E080..U+E0FF 字面字符 3 字节→1 字节静默丢失)。
