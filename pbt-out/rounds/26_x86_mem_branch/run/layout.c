#include <stdio.h>
#include <stddef.h>
union UV {
    struct { unsigned a : 3, b : 5, c : 8, d : 16; } bits;
    unsigned raw;
    unsigned char bytes[4];
};
struct __attribute__((packed)) Mixed {
    char c;
    union UV u;
    short s;
};
struct Nested {
    struct { char a; int b; } inner;
    char tail;
};
int main(void) {
    union UV u = { 0 };
    u.bits.a = 7; u.bits.b = 31; u.bits.c = 255; u.bits.d = 65535;
    struct Mixed m = { 'X', u, -1 };
    struct Nested n = { { 'a', 42 }, 'z' };
    printf("%08x %d %d %d %zu\n", u.raw, u.bytes[0], u.bytes[3],
        m.s, sizeof(struct Mixed));
    printf("%c %d %c %zu %zu\n", n.inner.a, n.inner.b, n.tail,
        offsetof(struct Nested, inner.b), offsetof(struct Nested, tail));
    return 0;
}
