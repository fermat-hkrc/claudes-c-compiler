#include <stdio.h>
#include <string.h>

static int[4][3] g0 = {{583, 583, 1583}, {2583, 583}};
static unsigned char[16] g1 = { 160, 0, 96, 192, 32, 128, 224, 64, 160, 0, 96, 192, 32, 128, 224, 64 };
static int g2 = 796741;
static struct { unsigned a:3; unsigned b:5; int c:24; } g3 = {1, 25, -1524167};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g2=%llx %zu\n", (unsigned long long)g2, sizeof(g2));
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
