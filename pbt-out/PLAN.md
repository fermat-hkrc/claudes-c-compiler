# 性质测试(PBT)Campaign: claudes-c-compiler (ccc)

**Bug 上报策略(用户指示):** 确认 SUT bug 后,先用 `gh issue list` 检查现有 issue 是否重复 ——
重复则只写本地报告(`pbt-out/bug_reports/`);不重复则生成 issue 草稿,由用户决定是否发布。

## Scan findings

- **Test layout:** Rust 内联测试 —— src/ 下 499 个 `#[test]`,位于各源文件的
  `#[cfg(test)] mod tests` 块(`src/common/encoding.rs` 已有;`const_arith.rs`/`types.rs` 尚无)。
  无 `tests/` 目录,Cargo.toml 无 `[[test]]` 条目。运行器:`cargo test`。`src/lib.rs` 仅将
  `backend`/`driver` 声明为 `pub`(其余 `pub(crate)`),故单元级性质测试内联写入 src 源文件;
  E2E 性质测试放入新建集成测试 `tests/e2e_diff.rs`(经 `env!("CARGO_BIN_EXE_ccc")` 调用编译器)。
- **Buildability probe:** `cargo build --release` → **通过**,46.65s(仅 1 个既有 warning)。
  冒烟:`./target/release/ccc /tmp/smoke1.c -o /tmp/smoke1 && /tmp/smoke1` → 输出 `hello 42`,exit 0。
  E2E 计时:ccc 0.20s vs gcc 0.58s(在 /mnt/c 上;/tmp 更快)。gcc 13.3 可用。
- **Harness placement:** Rung 1(第 1 档)—— 扩展仓库自身测试设施:添加 `proptest` 为
  dev-dependency;在 `src/common/{encoding,const_arith,types}.rs` 内新增内联测试模块;新增集成测试
  `tests/e2e_diff.rs`,统一经 `cargo test` 运行。不改动生产代码。
- **Candidate modules:**
  - `e2e_diff` —— 完整 driver→lexer→parser→sema→IR→optimizer→codegen→assembler→linker 流水线;
    oracle:与同机 gcc-13.3 差分对比(各自编译并运行,比较 stdout+exit code)。可用最强 oracle。
  - `common/encoding.rs` —— PUA 字节↔PUA 码点往返(algebraic)。
  - `common/const_arith.rs` —— 常量折叠整数二元运算 vs C 语义参照模型(differential)。
  - `common/types.rs` —— `align_up` 定律与结构体布局不变式(algebraic)。
- **Skipped modules:** backend/{arm,riscv,i686} 的 codegen/assembler(需交叉 sysroot 与 qemu 才能运行;
  x86-64 E2E 已原生覆盖完整流水线);`long_double.rs` f128 软浮点(独立数值子系统,预算内优先级
  较低);preprocessor 宏展开(E2E 已部分覆盖);passes/* 优化器内部(默认优化级别下由 E2E 差分
  间接覆盖)。
- **Spec evidence:** DESIGN_DOC.md:164 "x86-64 code generation (SysV AMD64 ABI)";README.md;
  encoding.rs 模块文档定义 PUA 往返契约;C11 整数语义为常量折叠外部参照。

## Module: e2e_diff
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Module: common::encoding
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Module: common::const_arith
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Module: common::types
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
