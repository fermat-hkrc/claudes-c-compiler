#include <stdio.h>
#include <string.h>

static union { long l; short s[4]; } g0 = {0x5fe354881d633f8b};
static unsigned char[16] g1 = { 139, 216, 37, 114, 191, 12, 89, 166, 243, 64, 141, 218, 39, 116, 193, 14 };
static long long g2 = 2035430177u;
static unsigned char[16] g3 = { 157, 104, 51, 254, 201, 148, 95, 42, 245, 192, 139, 86, 33, 236, 183, 130 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g2=%llx %zu\n", (unsigned long long)g2, sizeof(g2));
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
