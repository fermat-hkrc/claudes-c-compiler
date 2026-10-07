#include <stdio.h>
#include <string.h>

static short[5] g0 = {1832, 32, 31832};
static int[4][3] g1 = {{737, 737, 1737}, {2737, 3737}};
static unsigned char[16] g2 = { 103, 184, 9, 90, 171, 252, 77, 158, 239, 64, 145, 226, 51, 132, 213, 38 };
static union { int i; char c[8]; double d; } g3 = {0xa60a172c};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
