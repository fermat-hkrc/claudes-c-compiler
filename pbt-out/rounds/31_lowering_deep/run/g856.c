#include <stdio.h>
#include <string.h>

static struct { int a; char b; double c; } g0 = {46298, 'q', 4208.909090909091};
static short[5] g1 = {4857, 57, 34857};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'w', 'x', 'y'}, 2092, {22092, 2989588496161102092}};
static long long g3 = 1443296384u;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
