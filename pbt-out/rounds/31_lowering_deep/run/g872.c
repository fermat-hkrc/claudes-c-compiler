#include <stdio.h>
#include <string.h>

static int g0 = -127921;
static union { long l; short s[4]; } g1 = {0x3d3886fbc3981d34};
static char[8] g2 = {'t', 'T', '', 105};
static union { int i; char c[8]; double d; } g3 = {0xe63a4be1};

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
