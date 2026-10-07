#include <stdio.h>
#include <string.h>

static long long g0 = 611683600u;
static int[4][3] g1 = {{914, 1914, 1914}, {3914, 3914}};
static short[5] g2 = {29726, 26, 9726};
static unsigned char[16] g3 = { 214, 176, 138, 100, 62, 24, 242, 204, 166, 128, 90, 52, 14, 232, 194, 156 };

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
