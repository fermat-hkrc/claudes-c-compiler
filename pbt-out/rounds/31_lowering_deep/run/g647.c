#include <stdio.h>
#include <string.h>

static int[4][3] g0 = {{632, 1632, 1632}, {1632, 1632}};
static short[5] g1 = {18833, 33, 58833};
static struct { int a; char b; double c; } g2 = {37679, 'v', 3425.3636363636365};
static int g3 = -29434;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
