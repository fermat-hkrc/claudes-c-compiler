#include <stdio.h>
#include <string.h>

static union { int i; char c[8]; double d; } g0 = {0x70ba0ed0};
static double g1 = 'p';
static unsigned char[16] g2 = { 161, 136, 111, 86, 61, 36, 11, 242, 217, 192, 167, 142, 117, 92, 67, 42 };
static unsigned char[16] g3 = { 141, 232, 67, 158, 249, 84, 175, 10, 101, 192, 27, 118, 209, 44, 135, 226 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
