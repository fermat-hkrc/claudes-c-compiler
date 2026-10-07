#include <stdio.h>
#include <string.h>

static union { int i; char c[8]; double d; } g0 = {0x201e14e8};
static short[5] g1 = {11550, 50, 41550};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'z', '{', '|'}, 5029, {15029, -3123885622197846587}};
static int g3 = -222325;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
