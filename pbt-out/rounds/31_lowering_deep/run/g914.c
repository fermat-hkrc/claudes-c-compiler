#include <stdio.h>
#include <string.h>

static char[8] g0 = {'m', 'M', '', 60};
static long long g1 = 1786400439u;
static int[4][3] g2 = {{456, 1456, 456}, {1456, 456}};
static struct { int a; char b; double c; } g3 = {56882, 'u', 5171.090909090909};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
