#include <stdio.h>
#include <string.h>

static union { long l; short s[4]; } g0 = {0xd65cee800272abc};
static struct { char x[3]; int y; struct { short p; long q; } z; } g1 = {{'m', 'n', 'o'}, 85434, {5434, 4299234778402985434}};
static unsigned char[16] g2 = { 224, 0, 32, 64, 96, 128, 160, 192, 224, 0, 32, 64, 96, 128, 160, 192 };
static char[8] g3 = {'y', 'Y', '', 72};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
