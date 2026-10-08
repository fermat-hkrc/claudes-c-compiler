# octal literals containing digits 8 or 9 are accepted as DECIMALS — 018 reads as 18 (gcc: 'invalid digit 8 in octal constant')
> #1175
# duplicate/mixed-order integer suffixes accepted — 42luL, 42uu, 42lll, 42lul compile clean (C11 6.4.2.1: at most one u + one l/ll, order-constrained)
> #1176
# contradictory type specifiers accepted — signed unsigned, long short, short long, static auto, typedef static all compile clean (C11 6.7.2p2 + 6.7.1p2)
> #1177
# empty character constant '' accepted, value 0 (gcc: 'empty character constant')
> #1178
# floating literal '1e' with empty exponent accepted, value 0 (gcc: 'exponent has no digits')
> #1179
# hex literals 0x (no digits) and 0x1p (p-marker with no exponent digits) accepted — values 0 and 1
> #1180
# no tail-call optimization — 1M-deep tail recursion SEGFAULTS where gcc -O2 returns normally (functional-style C crashes)
> #1181
