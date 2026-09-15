# PROPERTIES(性质清单)

> 状态流转:proposed → approved → passing | failing | retired

## P1 e2e_arith_expr —— 整数表达式求值等价
- Tier: 1(差分,编译器可用的最强 oracle)
- 理由: DESIGN_DOC.md:164 声明 "x86-64 code generation (SysV AMD64 ABI)";C 编译器的契约是生成
  程序按 C 标准语义计算。独立参照:同机 gcc-13.3 编译同一程序。参照 KAT 门:冒烟测试
  `hello 42` 通过(见 PLAN.md 探针)。
- Formal: ∀ p ∈ ArithExprDSL(良定义 C:无 UB). run(ccc(p)) = run(gcc(p)),其中 run = (stdout, exit_code)
- Test file: tests/e2e_diff.rs
- Status: proposed
- Counterexample: (none)
- Bug report: (none)

```property
function: ccc::driver::compiler_main (E2E pipeline)
oracle: differential
predicate:
  relation: { op: eq, lhs: "run(ccc_compile_and_exec(p))", rhs: "run(gcc_compile_and_exec(p))" }
generators:
  p: { gen: custom, dsl: arithmetic expression tree over {+,-,*,/,%,<<,>>,&,|,^,cmp,casts} on int/unsigned/long/unsigned-long with UB-guarded operand domains }
evidence: DESIGN_DOC.md:164 "x86-64 code generation (SysV AMD64 ABI)"; README.md "A C compiler ... produces ELF executables"
```

## P2 e2e_control_flow —— 循环/分支输出等价
- Tier: 1
- 理由: 同上差分契约;控制流考验 CFG 构建、跨循环寄存器分配、分支代码生成。
- Formal: ∀ p ∈ ControlFlowDSL(嵌套 if/while/for + break/continue,终止上界有界).
  run(ccc(p)) = run(gcc(p))
- Test file: tests/e2e_diff.rs
- Status: proposed
- Counterexample: (none)
- Bug report: (none)

```property
function: ccc::driver::compiler_main (E2E pipeline)
oracle: differential
predicate:
  relation: { op: eq, lhs: "run(ccc_compile_and_exec(p))", rhs: "run(gcc_compile_and_exec(p))" }
generators:
  p: { gen: custom, dsl: bounded nested control flow with induction vars }
evidence: DESIGN_DOC.md:164
```

## P3 e2e_struct_layout —— sizeof/offsetof/内存模型等价
- Tier: 1
- 理由: 结构体布局与内存模型必须匹配平台 ABI(SysV);通过生成的程序观测 sizeof、指针差偏移、
  字段读写来检验。
- Formal: ∀ p ∈ StructDSL(混合类型字段、数组、嵌套、联合). run(ccc(p)) = run(gcc(p))
- Test file: tests/e2e_diff.rs
- Status: proposed
- Counterexample: (none)
- Bug report: (none)

```property
function: ccc::driver::compiler_main (E2E pipeline)
oracle: differential
predicate:
  relation: { op: eq, lhs: "run(ccc_compile_and_exec(p))", rhs: "run(gcc_compile_and_exec(p))" }
generators:
  p: { gen: custom, dsl: struct/union/array layout probe program }
evidence: DESIGN_DOC.md:164 (SysV ABI); src/common/types.rs 布局文档注释
```

## P4 e2e_calls_abi —— 函数调用、>6 参数、按值传结构体、递归、函数指针
- Tier: 1
- 理由: SysV 调用约定是设计文档明确声明的契约;寄存器/栈传参、结构体传参分类是经典的
  误编译高发区。
- Formal: ∀ p ∈ CallDSL. run(ccc(p)) = run(gcc(p))
- Test file: tests/e2e_diff.rs
- Status: proposed
- Counterexample: (none)
- Bug report: (none)

```property
function: ccc::driver::compiler_main (E2E pipeline)
oracle: differential
predicate:
  relation: { op: eq, lhs: "run(ccc_compile_and_exec(p))", rhs: "run(gcc_compile_and_exec(p))" }
generators:
  p: { gen: custom, dsl: call/return shapes incl. 0..10 args, by-value small structs, recursion, fn ptrs }
evidence: DESIGN_DOC.md:151 call_abi.rs "Unified ABI classification"; DESIGN_DOC.md:164
```

## P5 e2e_globals_init —— 全局变量/静态变量/数组带初始化器
- Tier: 1
- 理由: .data/.bss 布局、初始化器折叠、静态局部变量 —— 链接器 + 常量求值面。
- Formal: ∀ p ∈ GlobalDSL. run(ccc(p)) = run(gcc(p))
- Test file: tests/e2e_diff.rs
- Status: proposed
- Counterexample: (none)
- Bug report: (none)

```property
function: ccc::driver::compiler_main (E2E pipeline)
oracle: differential
predicate:
  relation: { op: eq, lhs: "run(ccc_compile_and_exec(p))", rhs: "run(gcc_compile_and_exec(p))" }
generators:
  p: { gen: custom, dsl: global scalars/arrays/structs with const initializers, static locals }
evidence: DESIGN_DOC.md(linker/ELF writer 章节)
```

## P6 encoding_roundtrip —— bytes_to_string ∘ decode_pua_byte = id
- Tier: 3(代数往返;模块文档注释声明了契约)
- 理由: encoding.rs:8-10 "encode non-UTF-8 bytes using ... then decode them back to raw bytes"。
  更强候选被否决:无参照实现;纯函数不适用状态机。
- Formal: ∀ b ∈ Vec<u8>, ¬b.starts_with(BOM) ⇒ decode_all(bytes_to_string(b).bytes()) = b
- Test file: src/common/encoding.rs(内联)
- Status: proposed
- Counterexample: (none)
- Bug report: (none)

```property
function: ccc::common::encoding::{bytes_to_string, decode_pua_byte}
oracle: algebraic.round_trip
round_trip: { forward: bytes_to_string, backward: decode_pua_byte_stream, var: bytes }
generators:
  bytes: { gen: list, elem: { gen: int, min: 0, max: 255, type: u8 }, maxLen: 64 }
evidence: src/common/encoding.rs:1-10 模块文档: "encode ... then decode them back to raw bytes"
```

## P7 encoding_idempotent —— 编码在再编码下稳定
- Tier: 4
- 理由: 编码结果本身是合法 UTF-8;对其字节再编码必须是无操作(文档:"If the bytes are valid
  UTF-8, returns them as-is")。
- Formal: ∀ b ∈ Vec<u8>, ¬b.starts_with(BOM) ⇒ bytes_to_string(bytes_to_string(b).bytes().to_vec()) = bytes_to_string(b)
- Test file: src/common/encoding.rs(内联)
- Status: proposed
- Counterexample: (none)
- Bug report: (none)

```property
function: ccc::common::encoding::bytes_to_string
oracle: algebraic.idempotence
idempotence: { function: bytes_to_string }
generators:
  b: { gen: list, elem: { gen: int, min: 0, max: 255, type: u8 }, maxLen: 64 }
evidence: src/common/encoding.rs:16 "If the bytes are valid UTF-8, returns them as-is."
```

## P8 const_arith_matches_c_semantics —— 常量折叠整数二元运算 vs C 参照模型
- Tier: 2(差分:独立 C 语义参照模型)
- 理由: const_arith.rs:1-9 "compile-time constant expression evaluation with proper C semantics"。
  参照 = 以原生运算实现的 C11 整数提升/转换规则(独立于 SUT 代码)。生成器偏斜至边界:
  0、±1、INT_MAX/MIN、UINT_MAX、LONG_MIN/MAX、移位数 0/1/31/32/63;排除 i64::MIN/-1(C 中 UB)。
- Formal: ∀ (op,l,r,w∈{32,64},s∈{signed,unsigned}) ∈ DefinedDomain.
  eval_const_binop(op,l,r,w,s) = c_reference(op,l,r,w,s)
- Test file: src/common/const_arith.rs(内联,新建测试模块)
- Status: proposed
- Counterexample: (none)
- Bug report: (none)

```property
function: ccc::common::const_arith::eval_const_binop
oracle: differential
predicate:
  relation: { op: eq, lhs: "eval_const_binop(op, l, r, is_32bit, is_unsigned)", rhs: "c_ref_model(op, l, r, is_32bit, is_unsigned)" }
generators:
  op: { gen: oneof, of: [add, sub, mul, div, mod, shl, shr, and, or, xor, eq, ne, lt, gt, le, ge] }
  l: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64, skew: boundaries }
  r: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64, skew: boundaries }
evidence: src/common/const_arith.rs:1-9 "compile-time constant expression evaluation with proper C semantics"
```

## P9 align_up_laws —— 对齐取整为最小倍数
- Tier: 3(代数不变式)
- 理由: align_up 被所有结构体/栈偏移使用;C 对齐语义要求精确的下一个倍数。定律:r ≥ x、
  r % align == 0、最小性(中间无更小的合法倍数)、align==0 直通(溢出分支原样返回)。
- Formal: ∀ x ≥ 0, a ≥ 1, x + a - 1 ≤ usize::MAX ⇒ r = align_up(x,a): r ≥ x ∧ a | r ∧ r - a < x
- Test file: src/common/types.rs(内联,新建测试模块)
- Status: proposed
- Counterexample: (none)
- Bug report: (none)

```property
function: ccc::common::types::align_up
oracle: algebraic.invariant
predicate:
  relation:
    op: holds
    expr: "let r = align_up(x, a); r >= x && r % a == 0 && (r - a) < x"
generators:
  x: { gen: int, min: 0, max: 18446744073709551615, type: usize }
  a: { gen: int, min: 1, max: 4096, type: usize }
evidence: src/common/types.rs:1177(align_up 定义;被全部结构体布局代码使用)
```
