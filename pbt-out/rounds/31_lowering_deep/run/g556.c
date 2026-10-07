#include <stdio.h>
#include <string.h>

static double g0 = 'w';
static struct { unsigned a:3; unsigned b:5; int c:24; } g1 = {2, 2, 2410626};
static unsigned char[16] g2 = { 253, 104, 211, 62, 169, 20, 127, 234, 85, 192, 43, 150, 1, 108, 215, 66 };
static struct { int a; char b; double c; } g3 = {85545, 't', 7776.818181818182};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
