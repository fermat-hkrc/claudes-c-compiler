#include <stdio.h>
#include <string.h>

static struct { unsigned x:1; unsigned y:31; } g0 = {0, 4749964787223071720};
static long long g1 = 3193525335u;
static unsigned char[16] g2 = { 167, 184, 201, 218, 235, 252, 13, 30, 47, 64, 81, 98, 115, 132, 149, 166 };
static unsigned char[16] g3 = { 32, 0, 224, 192, 160, 128, 96, 64, 32, 0, 224, 192, 160, 128, 96, 64 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
