#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 213, 40, 123, 206, 33, 116, 199, 26, 109, 192, 19, 102, 185, 12, 95, 178 };
static int g1 = -702210;
static short[5] g2 = {19349, 49, 59349};
static int g3 = 114275;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
