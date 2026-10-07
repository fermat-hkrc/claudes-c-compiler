#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 79, 248, 161, 74, 243, 156, 69, 238, 151, 64, 233, 146, 59, 228, 141, 54 };
static union { int i; char c[8]; double d; } g1 = {0x9b1df85f};
static unsigned char[16] g2 = { 70, 48, 26, 4, 238, 216, 194, 172, 150, 128, 106, 84, 62, 40, 18, 252 };
static short[5] g3 = {21051, 51, 51051};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
