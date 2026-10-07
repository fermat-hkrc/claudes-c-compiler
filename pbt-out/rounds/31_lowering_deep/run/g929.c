#include <stdio.h>
#include <string.h>

static double g0 = 'x';
static unsigned char[16] g1 = { 207, 248, 33, 74, 115, 156, 197, 238, 23, 64, 105, 146, 187, 228, 13, 54 };
static struct { unsigned a:3; unsigned b:5; int c:24; } g2 = {1, 25, -4011175};
static union { int i; char c[8]; double d; } g3 = {0xf3c6d538};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
