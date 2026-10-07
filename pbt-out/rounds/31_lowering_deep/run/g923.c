#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 27, 88, 149, 210, 15, 76, 137, 198, 3, 64, 125, 186, 247, 52, 113, 174 };
static short[5] g1 = {11130, 30, 21130};
static struct { int a; char b; double c; } g2 = {10788, '', 980.7272727272727};
static int g3 = -586018;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
