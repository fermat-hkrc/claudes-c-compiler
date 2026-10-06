#include <stdio.h>
#include <setjmp.h>
#include <string.h>
struct Big { char data[256]; int id; };
static jmp_buf env;
static struct Big global_big;
int deep_call(int depth) {
    struct Big local = { 0 };
    local.id = depth;
    if (depth == 0) { memcpy(&global_big, &local, sizeof(local)); longjmp(env, 42); }
    return deep_call(depth - 1) + 1;
}
int main(void) {
    int v = setjmp(env);
    if (v == 0) { deep_call(50); return 1; }
    printf("%d %d\n", v, global_big.id);
    // 指针运算极端
    char buf[100];
    char *p = buf + 99;
    *p = 'Z';
    printf("%c %ld\n", *p, (long)(p - buf));
    return 0;
}
