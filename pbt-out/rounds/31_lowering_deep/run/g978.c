#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 62, 240, 162, 84, 6, 184, 106, 28, 206, 128, 50, 228, 150, 72, 250, 172 };
static int g1 = 484818;
static unsigned char[16] g2 = { 190, 240, 34, 84, 134, 184, 234, 28, 78, 128, 178, 228, 22, 72, 122, 172 };
static long long g3 = 679641557u;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
