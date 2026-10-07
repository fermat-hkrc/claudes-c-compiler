# empty hex escapes accepted — "\x" and "\xg" compile clean; gcc errors (C11 6.4.4.4: hex-escape requires ≥1 hex digit)
> #1156
# -U does not cancel -D
> #1157
# function-like -D is unsupported — -D 'MAX(a,b)=((a)>(b)?(a):(b))' leaves MAX an identifier ('implicit declaration of function MAX')
> #1158
# self-including header builds SILENTLY — #include recursion terminated without diagnostic while gcc errors '#include nested too deeply'
> #1159
# -E -C is silently ignored — comments still stripped (gcc -C retains them)
> #1160
# -M dependency output omits ALL included headers — only the main .c is listed (make never rebuilds on header changes)
> #1161
# -MF is ignored — dependency rule goes to STDOUT instead of the named file (gcc -MF FILE writes FILE)
> #1162
# -g is silently ignored — zero DWARF/debug sections emitted (gcc: 8); ccc binaries have neither symbols (#1092) nor debug info, gdb- unusable
> #1163
# CPATH ignored
> #1164
