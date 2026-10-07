#include <stdio.h>
#include <string.h>

static union { int i; char c[8]; double d; } g0 = {0x3298b1a5};
static int g1 = 160797;
static struct { unsigned x:1; unsigned y:31; } g2 = {0, 10229796609529823486};
static unsigned char[16] g3 = { 24, 192, 104, 16, 184, 96, 8, 176, 88, 0, 168, 80, 248, 160, 72, 240 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
