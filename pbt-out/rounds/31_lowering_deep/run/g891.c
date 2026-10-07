#include <stdio.h>
#include <string.h>

static union { long l; short s[4]; } g0 = {0xf23e4c0c53fc9db9};
static struct { char x[3]; int y; struct { short p; long q; } z; } g1 = {{'', '', ''}, 35671, {15671, -7106159494694315945}};
static short[5] g2 = {4647, 47, 14647};
static unsigned char[16] g3 = { 80, 128, 176, 224, 16, 64, 112, 160, 208, 0, 48, 96, 144, 192, 240, 32 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
