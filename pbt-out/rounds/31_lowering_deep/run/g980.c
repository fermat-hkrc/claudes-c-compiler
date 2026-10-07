#include <stdio.h>
#include <string.h>

static int g0 = -171772;
static short[5] g1 = {15194, 94, 25194};
static struct { unsigned a:3; unsigned b:5; int c:24; } g2 = {1, 17, -1000655};
static struct { char x[3]; int y; struct { short p; long q; } z; } g3 = {{'n', 'o', 'p'}, 78405, {8405, 5463220896438878405}};

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
