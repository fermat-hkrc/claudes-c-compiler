#include <stdio.h>
#include <string.h>

static char[8] g0 = {'a', 'A', 'x', 76};
static struct { int a; char b; double c; } g1 = {58994, 'y', 5363.090909090909};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'v', 'w', 'x'}, 25955, {25955, -5417891906331025661}};
static struct { unsigned x:1; unsigned y:31; } g3 = {1, 17544806041309031723};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
