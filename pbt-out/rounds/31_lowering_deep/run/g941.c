#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 189, 104, 19, 190, 105, 20, 191, 106, 21, 192, 107, 22, 193, 108, 23, 194 };
static union { int i; char c[8]; double d; } g1 = {0xf12c9435};
static union { int i; char c[8]; double d; } g2 = {0x5bd0086a};
static union { int i; char c[8]; double d; } g3 = {0xfcb9b5d4};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
