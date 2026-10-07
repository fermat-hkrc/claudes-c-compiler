#include <stdio.h>
#include <string.h>

static union { long l; short s[4]; } g0 = {0xf0c85da35db0b6e4};
static union { long l; short s[4]; } g1 = {0x511e494e0a7f483d};
static union { long l; short s[4]; } g2 = {0x9e987c29c4e876b0};
static unsigned char[16] g3 = { 254, 240, 226, 212, 198, 184, 170, 156, 142, 128, 114, 100, 86, 72, 58, 44 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
