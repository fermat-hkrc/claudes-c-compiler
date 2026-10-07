#include <stdio.h>
#include <string.h>

static union { long l; short s[4]; } g0 = {0xa9ac65641e47465d};
static unsigned char[16] g1 = { 141, 232, 67, 158, 249, 84, 175, 10, 101, 192, 27, 118, 209, 44, 135, 226 };
static struct { int a; char b; double c; } g2 = {52439, '|', 4767.181818181818};
static union { int i; char c[8]; double d; } g3 = {0x13effc8a};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
