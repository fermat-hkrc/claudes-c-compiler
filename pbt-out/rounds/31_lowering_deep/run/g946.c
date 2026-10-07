#include <stdio.h>
#include <string.h>

static long long g0 = 3016903354u;
static union { long l; short s[4]; } g1 = {0x20ae2212f266f165};
static unsigned char[16] g2 = { 226, 16, 62, 108, 154, 200, 246, 36, 82, 128, 174, 220, 10, 56, 102, 148 };
static short[5] g3 = {20091, 91, 10091};

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
