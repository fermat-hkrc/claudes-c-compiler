#include <stdio.h>
#include <string.h>

static union { int i; char c[8]; double d; } g0 = {0xc36ef7b2};
static struct { char x[3]; int y; struct { short p; long q; } z; } g1 = {{'w', 'x', 'y'}, 60200, {200, 6466660110186960200}};
static char[8] g2 = {'c', 'C', 'z', 220};
static struct { unsigned x:1; unsigned y:31; } g3 = {1, 9671223668086126083};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
