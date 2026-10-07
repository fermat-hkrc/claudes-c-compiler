# Block-scope static function definition accepted
> **Filed as #1150**
gcc: "invalid storage class for function"; clang: "function definition is not allowed here"; ccc compiles.
Found by round-42 memgraph fuzzer bring-up (gcc rejected generated programs).
