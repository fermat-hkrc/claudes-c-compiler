#include <stdio.h>
#include <string.h>

static int g0 = -458704;
static union { int i; char c[8]; double d; } g1 = {0x83c33a18};
static unsigned char[16] g2 = { 150, 176, 202, 228, 254, 24, 50, 76, 102, 128, 154, 180, 206, 232, 2, 28 };
static long long g3 = 786168114u;

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
