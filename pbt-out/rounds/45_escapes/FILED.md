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
