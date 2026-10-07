#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 115, 24, 189, 98, 7, 172, 81, 246, 155, 64, 229, 138, 47, 212, 121, 30 };
static struct { unsigned a:3; unsigned b:5; int c:24; } g1 = {0, 24, -3680424};
static double g2 = 't';
static struct { unsigned x:1; unsigned y:31; } g3 = {0, 3523936272365965880};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
