#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 193, 136, 79, 22, 221, 164, 107, 50, 249, 192, 135, 78, 21, 220, 163, 106 };
static union { long l; short s[4]; } g1 = {0xdc0b81a5b526092b};
static long long g2 = 1729059612u;
static union { long l; short s[4]; } g3 = {0x8feeabe3b300c0e4};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g2=%llx %zu\n", (unsigned long long)g2, sizeof(g2));
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
