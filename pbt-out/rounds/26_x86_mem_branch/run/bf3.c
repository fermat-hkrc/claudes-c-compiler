#include <stdio.h>
struct X {
    int a : 1;
    unsigned b : 1;
    int c : 31;
    unsigned d : 32;
    int : 0;         // 匿名零宽——合法
    unsigned f : 5;
};
int main(void) {
    struct X x = { -1, 1, 0x7FFFFFFF, 0xFFFFFFFF, 31 };
    printf("%d %u %d %u %u %d %zu\n",
        x.a, x.b, x.c, x.d, x.f, x.a == -1, sizeof(struct X));
    return 0;
}
