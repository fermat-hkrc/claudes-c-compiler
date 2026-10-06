#include <stdio.h>
struct X {
    int a : 1;       // 1-bit signed: -1 or 0
    unsigned b : 1;
    int c : 31;      // crosses into next word
    unsigned d : 32; // full word
    int e : 0;       // zero-width: force alignment
    unsigned f : 5;
};
int main(void) {
    struct X x = { -1, 1, 0x7FFFFFFF, 0xFFFFFFFF, 0, 31 };
    printf("%d %u %d %u %u %d %zu\n",
        x.a, x.b, x.c, x.d, x.f, x.a == -1, sizeof(struct X));
    return 0;
}
