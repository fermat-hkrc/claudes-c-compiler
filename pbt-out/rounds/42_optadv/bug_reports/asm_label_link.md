# asm symbol-rename labels break the link — int f(void) asm("other"); emits the call under the new name but the definition under the old (undefined symbol)
> **Filed as [#1152](.../issues/1152)**
