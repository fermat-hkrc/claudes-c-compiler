#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 116, 160, 204, 248, 36, 80, 124, 168, 212, 0, 44, 88, 132, 176, 220, 8 };
static long long g1 = 1930689606u;
static struct { unsigned a:3; unsigned b:5; int c:24; } g2 = {4, 20, -843756};
static int[4][3] g3 = {{503, 503, 1503}, {503, 503}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
