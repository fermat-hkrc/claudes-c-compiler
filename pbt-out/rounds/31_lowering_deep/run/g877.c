#include <stdio.h>
#include <string.h>

static int[4][3] g0 = {{234, 1234, 2234}, {3234, 4234}};
static unsigned char[16] g1 = { 208, 128, 48, 224, 144, 64, 240, 160, 80, 0, 176, 96, 16, 192, 112, 32 };
static struct { unsigned a:3; unsigned b:5; int c:24; } g2 = {0, 16, 201520};
static int g3 = -514141;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
