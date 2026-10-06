# Plan

- [ ] 探 ARM 非 NEON 面的扩展族与验证行为（超出第 10 轮的 15 族）
- [ ] 建 p1 合法差分 + p2 变异拒绝（双参考：llvm-mc + aarch64-linux-gnu-as）
- [ ] 跑满 400+400，收集全部失败类别
- [ ] 双参考终审 + 对照队友 263 题清单去重
- [ ] elf 深挖：resolve_numeric_labels 的 GNU 1f/1b 语义属性
- [ ] 发 issue、红回归、收尾提交
