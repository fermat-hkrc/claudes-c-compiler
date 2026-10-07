#include <stdio.h>
#include <string.h>

static int[4][3] g0 = {{759, 1759, 759}, {1759, 2759}};
static unsigned char[16] g1 = { 179, 24, 125, 226, 71, 172, 17, 118, 219, 64, 165, 10, 111, 212, 57, 158 };
static int g2 = -315580;
static char[8] g3 = {'s', 'S', '', 180};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g2=%llx %zu\n", (unsigned long long)g2, sizeof(g2));
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
