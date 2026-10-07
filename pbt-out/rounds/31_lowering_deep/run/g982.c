#include <stdio.h>
#include <string.h>

static int g0 = 255849;
static short[5] g1 = {13120, 20, 23120};
static int g2 = -839130;
static unsigned char[16] g3 = { 117, 40, 219, 142, 65, 244, 167, 90, 13, 192, 115, 38, 217, 140, 63, 242 };

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g2=%llx %zu\n", (unsigned long long)g2, sizeof(g2));
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
