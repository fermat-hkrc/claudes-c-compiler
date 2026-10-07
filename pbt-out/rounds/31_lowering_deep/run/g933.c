#include <stdio.h>
#include <string.h>

static int[4][3] g0 = {{697, 697, 1697}, {2697, 4697}};
static struct { int a; char b; double c; } g1 = {21766, '', 1978.7272727272727};
static unsigned char[16] g2 = { 177, 8, 95, 182, 13, 100, 187, 18, 105, 192, 23, 110, 197, 28, 115, 202 };
static short[5] g3 = {16845, 45, 46845};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
