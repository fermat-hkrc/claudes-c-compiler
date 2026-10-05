# Plan

- [x] 建立 issue 去重数据库（拉取全部 ~500 个 issue 的编号/标题/函数名到 pbt-out/issue_db.tsv）
- [x] 创建输出目录 pbt-out/bug_reports/（新 bug）和 pbt-out/bug_reports/found_already/（已有 issue 的重复 bug），编写 issue 建单辅助脚本
- [x] PBT 战役：src/common（encoding/const_eval/const_arith/long_double 等）
- [x] PBT 战役：src/frontend lexer ✓ / preprocessor ✓（parser、sema 继续按子目录轮次）
- [ ] PBT 战役：src/ir + src/passes
- [ ] PBT 战役：src/backend 核心（cast/regalloc/liveness/stack_layout/call_abi）
- [ ] PBT 战役：src/backend/elf + linker_common
- [ ] PBT 战役：src/backend/x86 + x86_common + i686
- [ ] PBT 战役：src/backend/riscv
- [ ] PBT 战役：src/backend/arm（去重压力最大，已有大量 encode_* issue）
- [ ] PBT 战役：src/driver + 剩余未覆盖文件
- [x] 每轮战役后：bug report 去重分流（流程已建立：复现验证→正文比对→建单/归档） → 新 bug 建 GitHub issue（遵循 issue 格式）/ 重复移入 found_already
- [ ] 最终汇总（pbt-out/bug_reports/SUMMARY.md）
