#include <stdio.h>
#include <string.h>

static int[4][3] g0 = {{416, 1416, 1416}, {3416, 3416}};
static struct { unsigned x:1; unsigned y:31; } g1 = {0, 8484220022697848938};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'y', 'z', '{'}, 75382, {15382, -8491546755016776234}};
static int g3 = -188211;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
