#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 252, 224, 196, 168, 140, 112, 84, 56, 28, 0, 228, 200, 172, 144, 116, 88 };
static double g1 = 'r';
static unsigned char[16] g2 = { 134, 48, 218, 132, 46, 216, 130, 44, 214, 128, 42, 212, 126, 40, 210, 124 };
static union { int i; char c[8]; double d; } g3 = {0xab367fbe};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
