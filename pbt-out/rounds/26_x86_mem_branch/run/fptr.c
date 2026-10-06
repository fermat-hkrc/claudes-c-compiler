#include <stdio.h>
typedef long (*fn_t)(long, long);
typedef void (*vfn_t)(void);
long add(long a, long b) { return a + b; }
long mul(long a, long b) { return a * b; }
void nop_fn(void) { }
int main(void) {
    fn_t f = add;
    void *vp = (void*)f;
    f = (fn_t)vp;
    printf("%ld ", f(3, 4));
    f = mul;
    printf("%ld ", f(3, 4));
    // Tail position: calling through pointer then returning directly
    fn_t tbl[] = { add, mul, 0 };
    for (int i = 0; tbl[i]; i++)
        printf("%ld ", tbl[i](10, 20));
    vfn_t v = nop_fn;
    v();
    printf("\n");
    return 0;
}
