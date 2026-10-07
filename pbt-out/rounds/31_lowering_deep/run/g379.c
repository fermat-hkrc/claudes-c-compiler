#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 200, 64, 184, 48, 168, 32, 152, 16, 136, 0, 120, 240, 104, 224, 88, 208 };
static union { long l; short s[4]; } g1 = {0xdbc0e9f2819c9dd2};
static float g2 = 23078;
static unsigned char[16] g3 = { 99, 152, 205, 2, 55, 108, 161, 214, 11, 64, 117, 170, 223, 20, 73, 126 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
