#include <stdio.h>
#include <string.h>

static long long g0 = 1089618644u;
static unsigned char[16] g1 = { 78, 112, 146, 180, 214, 248, 26, 60, 94, 128, 162, 196, 230, 8, 42, 76 };
static union { long l; short s[4]; } g2 = {0xaed85ecdd2386a4c};
static struct { unsigned x:1; unsigned y:31; } g3 = {1, 15935645076325359429};

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
