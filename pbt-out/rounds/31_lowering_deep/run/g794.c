#include <stdio.h>
#include <string.h>

static struct { int a; char b; double c; } g0 = {21839, 't', 1985.3636363636363};
static char[8] g1 = {'r', 'R', '', 113};
static unsigned char[16] g2 = { 18, 144, 14, 140, 10, 136, 6, 132, 2, 128, 254, 124, 250, 120, 246, 116 };
static struct { int a; char b; double c; } g3 = {17070, 'u', 1551.8181818181818};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
