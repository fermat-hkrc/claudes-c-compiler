#include <stdio.h>
#include <string.h>

static int[4][3] g0 = {{854, 854, 1854}, {2854, 854}};
static int[4][3] g1 = {{694, 694, 1694}, {2694, 2694}};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'v', 'w', 'x'}, 78893, {28893, -7681379522473372723}};
static int[4][3] g3 = {{174, 174, 174}, {2174, 4174}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
