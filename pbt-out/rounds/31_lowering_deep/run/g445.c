#include <stdio.h>
#include <string.h>

static short[5] g0 = {473, 73, 50473};
static int[4][3] g1 = {{912, 1912, 2912}, {1912, 912}};
static struct { unsigned a:3; unsigned b:5; int c:24; } g2 = {4, 28, -1443556};
static long long g3 = 2522254111u;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
