#include <stdio.h>
#include <string.h>

static union { long l; short s[4]; } g0 = {0xd4e131b227e16755};
static unsigned char[16] g1 = { 181, 40, 155, 14, 129, 244, 103, 218, 77, 192, 51, 166, 25, 140, 255, 114 };
static union { long l; short s[4]; } g2 = {0x704767ae54110acb};
static union { long l; short s[4]; } g3 = {0x16c270c113c341bf};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
