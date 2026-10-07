#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 6, 48, 90, 132, 174, 216, 2, 44, 86, 128, 170, 212, 254, 40, 82, 124 };
static short[5] g1 = {23757, 57, 53757};
static union { long l; short s[4]; } g2 = {0xf35b2a471406ae28};
static unsigned char[16] g3 = { 249, 72, 151, 230, 53, 132, 211, 34, 113, 192, 15, 94, 173, 252, 75, 154 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
