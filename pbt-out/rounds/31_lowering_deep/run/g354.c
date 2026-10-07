#include <stdio.h>
#include <string.h>

static long long g0 = 817840521u;
static int[4][3] g1 = {{972, 1972, 1972}, {1972, 972}};
static union { long l; short s[4]; } g2 = {0x649f49eebe74d8c9};
static struct { char x[3]; int y; struct { short p; long q; } z; } g3 = {{'p', 'q', 'r'}, 32853, {2853, -7151951907150618763}};

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
