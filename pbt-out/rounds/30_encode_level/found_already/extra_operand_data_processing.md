# Extra-Operand Sweep: ARM data_processing — ALL DUPLICATES (2025-01 round 30)

Sweep result: 6 functions silently ignore extra operands. ALL already reported.
Old dedup DB was stale — refresh caught them.

| Function | Behavior | Duplicate of |
|----------|----------|-------------|
| encode_neg | 3rd operand ignored | #223 |
| encode_negs | 3rd operand ignored | #225 |
| encode_mvn | 3rd operand ignored | #215 |
| encode_mul | 4th operand ignored | #208 |
| encode_smull | 4th operand ignored | #276 |
| encode_madd | 5th operand ignored | #176 |

Teammate's recent wave (#1072 #1074 #1076 #1079 #1081) = same pattern on RISC-V functions.
Pattern fully covered. No new issue.
