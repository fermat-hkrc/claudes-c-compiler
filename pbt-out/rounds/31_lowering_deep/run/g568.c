#include <stdio.h>
#include <string.h>

static union { int i; char c[8]; double d; } g0 = {0xffab89b6};
static short[5] g1 = {10389, 89, 20389};
static unsigned char[16] g2 = { 30, 240, 194, 148, 102, 56, 10, 220, 174, 128, 82, 36, 246, 200, 154, 108 };
static struct { char x[3]; int y; struct { short p; long q; } z; } g3 = {{'v', 'w', 'x'}, 98139, {18139, 6177358011313398139}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
