#define EMPTY
#define FOO 1
#define CAT(a,b) a##b
#define S(x) #x
-EMPTY-
FOO + FOO
CAT(x, y)
S(a  b)
S("q\"t")
#if FOO > 0
yes
#else
no
#endif
