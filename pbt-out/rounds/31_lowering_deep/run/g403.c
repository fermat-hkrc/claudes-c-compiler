#include <stdio.h>
#include <string.h>

static struct { int a; char b; double c; } g0 = {59171, '', 5379.181818181818};
static int[4][3] g1 = {{835, 835, 835}, {835, 835}};
static char[8] g2 = {'q', 'Q', '', 220};
static unsigned char[16] g3 = { 168, 64, 216, 112, 8, 160, 56, 208, 104, 0, 152, 48, 200, 96, 248, 144 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
