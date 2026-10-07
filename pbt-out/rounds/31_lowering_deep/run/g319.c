#include <stdio.h>
#include <string.h>

static union { int i; char c[8]; double d; } g0 = {0x6b239b5f};
static char[8] g1 = {'w', 'W', '', 20};
static int g2 = 398627;
static unsigned char[16] g3 = { 179, 24, 125, 226, 71, 172, 17, 118, 219, 64, 165, 10, 111, 212, 57, 158 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g2=%llx %zu\n", (unsigned long long)g2, sizeof(g2));
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
