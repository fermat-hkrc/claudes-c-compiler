#include <stdio.h>
#include <string.h>

static struct { int a; char b; double c; } g0 = {92906, '{', 8446};
static unsigned char[16] g1 = { 169, 200, 231, 6, 37, 68, 99, 130, 161, 192, 223, 254, 29, 60, 91, 122 };
static int[4][3] g2 = {{10, 1010, 1010}, {3010, 10}};
static int[4][3] g3 = {{102, 102, 1102}, {102, 1102}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
