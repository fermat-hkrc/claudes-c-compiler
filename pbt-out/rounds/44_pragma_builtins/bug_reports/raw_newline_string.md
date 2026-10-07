# Raw newline in string literal accepted
> **Filed as #1155**
gcc: "missing terminating"; clang: 4 errors; ccc: exit 0 + object.
Found by round-44 generator accident (ccc compiled AND RAN the invalid source).
