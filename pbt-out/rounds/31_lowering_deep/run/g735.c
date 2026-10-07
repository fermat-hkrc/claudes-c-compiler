#include <stdio.h>
#include <string.h>

static char[8] g0 = {'y', 'Y', '', 72};
static struct { char x[3]; int y; struct { short p; long q; } z; } g1 = {{'}', '~', ''}, 21148, {1148, -4988751806155130468}};
static int[4][3] g2 = {{181, 1181, 1181}, {1181, 2181}};
static union { long l; short s[4]; } g3 = {0xdd02e097c1c5b0aa};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
