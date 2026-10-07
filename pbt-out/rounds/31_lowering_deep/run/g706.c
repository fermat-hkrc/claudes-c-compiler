#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 252, 224, 196, 168, 140, 112, 84, 56, 28, 0, 228, 200, 172, 144, 116, 88 };
static union { long l; short s[4]; } g1 = {0xa8557c51a59bcad8};
static unsigned char[16] g2 = { 213, 40, 123, 206, 33, 116, 199, 26, 109, 192, 19, 102, 185, 12, 95, 178 };
static union { int i; char c[8]; double d; } g3 = {0xeaa66702};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
