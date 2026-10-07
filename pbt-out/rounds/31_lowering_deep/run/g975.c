#include <stdio.h>
#include <string.h>

static int g0 = -171915;
static unsigned char[16] g1 = { 93, 104, 115, 126, 137, 148, 159, 170, 181, 192, 203, 214, 225, 236, 247, 2 };
static unsigned char[16] g2 = { 71, 184, 41, 154, 11, 124, 237, 94, 207, 64, 177, 34, 147, 4, 117, 230 };
static long long g3 = 1257130893u;

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
