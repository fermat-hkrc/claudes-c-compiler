#include <stdio.h>
#include <string.h>
static int *cp = (int[]){ 2094674356 % 97, 3238780028 % 89, 7, 8 };
static char *cs = (char[]){ 'h', 'i', 0 };
static struct N { int a[4]; struct { int x, y; } b; char t[3]; } n = {
    .a[1] = 2094674356 % 1000, .b.y = 3238780028 % 500, .a[3] = 99, .t[1] = 'Z', .b.x = -3
};
static struct { int u_sel; union { long l; char c[8]; } u; } au = { 5, 0x4142434445 };
static struct { unsigned p:5, q:11, r:16; } bf = { 2094674356 % 32, 3238780028 % 2048, 2094674356 % 65536 };
int count(void) {
    static int ctr = 0;
    static int tbl[3] = { 1873972513 % 100, 2, 3 };
    ctr++;
    return ctr * 10 + tbl[0] % 7;
}
int main(void) {
    printf("cp=%d %d %d %d\n", cp[0], cp[1], cp[2], cp[3]);
    printf("ca=%d %d\n", ca[1][0], ca[2][2]);
    printf("cs=%s\n", cs);
    unsigned char b[64]; memcpy(b, &n, sizeof(n));
    printf("n=%zu:", sizeof(n));
    for (unsigned i = 0; i < sizeof(n); i++) printf("%02x", b[i]);
    putchar('\n');
    memcpy(b, &au, sizeof(au));
    printf("au=%zu:", sizeof(au));
    for (unsigned i = 0; i < sizeof(au); i++) printf("%02x", b[i]);
    putchar('\n');
    memcpy(b, &bf, sizeof(bf));
    printf("bf=%zu:", sizeof(bf));
    for (unsigned i = 0; i < sizeof(bf); i++) printf("%02x", b[i]);
    putchar('\n');
    int r1 = count(); int r2 = count(); int r3 = count();
    printf("ctr=%d %d %d\n", r1, r2, r3);
    return 0;
}