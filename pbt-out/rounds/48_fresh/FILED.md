# enums with values beyond int are silently TRUNCATED to 4 bytes — enum { X = 0x100000000LL } reads as 0; gcc/clang widen the underlying type to 8
> #1173
