#include <stdio.h>
#include <string.h>

static char[8] g0 = {'v', 'V', '', 207};
static unsigned char[16] g1 = { 76, 96, 116, 136, 156, 176, 196, 216, 236, 0, 20, 40, 60, 80, 100, 120 };
static union { int i; char c[8]; double d; } g2 = {0x39a20210};
static long long g3 = 1524842687u;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
