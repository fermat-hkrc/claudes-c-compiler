#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 250, 208, 166, 124, 82, 40, 254, 212, 170, 128, 86, 44, 2, 216, 174, 132 };
static union { long l; short s[4]; } g1 = {0x815a944bd046ee62};
static unsigned char[16] g2 = { 234, 80, 182, 28, 130, 232, 78, 180, 26, 128, 230, 76, 178, 24, 126, 228 };
static union { long l; short s[4]; } g3 = {0x5e181a98032e8037};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
