#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 232, 64, 152, 240, 72, 160, 248, 80, 168, 0, 88, 176, 8, 96, 184, 16 };
static int g1 = -539990;
static float g2 = 30552;
static struct { unsigned x:1; unsigned y:31; } g3 = {1, 13188561007015757805};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
