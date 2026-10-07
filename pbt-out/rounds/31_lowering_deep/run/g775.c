#include <stdio.h>
#include <string.h>

static union { int i; char c[8]; double d; } g0 = {0x8e07ff56};
static struct { char x[3]; int y; struct { short p; long q; } z; } g1 = {{'w', 'x', 'y'}, 44696, {24696, 8906878154919144696}};
static unsigned char[16] g2 = { 247, 56, 121, 186, 251, 60, 125, 190, 255, 64, 129, 194, 3, 68, 133, 198 };
static double g3 = 'q';

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
