#include <stdio.h>
#include <string.h>

static struct { unsigned a:3; unsigned b:5; int c:24; } g0 = {7, 15, -7555441};
static union { int i; char c[8]; double d; } g1 = {0x75058e4b};
static struct { int a; char b; double c; } g2 = {1680, '', 152.72727272727272};
static unsigned char[16] g3 = { 82, 144, 206, 12, 74, 136, 198, 4, 66, 128, 190, 252, 58, 120, 182, 244 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
