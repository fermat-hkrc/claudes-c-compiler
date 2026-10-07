#include <stdio.h>
struct S_478 { float m0; float m1; };
extern struct S_478 mk(void);
extern unsigned long long take(struct S_478);
extern int verify_mk_ok;
int main(void) {
    struct S_478 a = mk();
    unsigned long long r1 = take(a);
    printf("take=%llx verify=%d sizeof=%zu\n", r1, verify_mk_ok, sizeof(struct S_478));
    struct S_478 c = mk();
    unsigned long long r2 = take(c);
    printf("ret=%llx\n", r2);
    return 0;
}