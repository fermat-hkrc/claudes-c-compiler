#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 166, 48, 186, 68, 206, 88, 226, 108, 246, 128, 10, 148, 30, 168, 50, 188 };
static long long g1 = 2687616012u;
static struct { unsigned x:1; unsigned y:31; } g2 = {0, 11465651296173631580};
static union { long l; short s[4]; } g3 = {0x2d20c999272b4635};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
