#include <stdio.h>
#include <string.h>

static struct { int a; char b; double c; } g0 = {73547, '', 6686.090909090909};
static short[5] g1 = {11662, 62, 1662};
static unsigned char[16] g2 = { 49, 8, 223, 182, 141, 100, 59, 18, 233, 192, 151, 110, 69, 28, 243, 202 };
static int g3 = 30273;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
