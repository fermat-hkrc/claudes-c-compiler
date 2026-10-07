#include <stdio.h>
#include <string.h>

static unsigned char[16] g0 = { 167, 184, 201, 218, 235, 252, 13, 30, 47, 64, 81, 98, 115, 132, 149, 166 };
static struct { int a; char b; double c; } g1 = {9419, '~', 856.2727272727273};
static struct { unsigned a:3; unsigned b:5; int c:24; } g2 = {0, 24, -3341640};
static struct { int a; char b; double c; } g3 = {51365, 'r', 4669.545454545455};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
