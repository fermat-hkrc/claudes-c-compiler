#include <stdio.h>
#include <string.h>

static struct { int a; char b; double c; } g0 = {73242, '', 6658.363636363636};
static struct { char x[3]; int y; struct { short p; long q; } z; } g1 = {{'~', '', ''}, 86293, {16293, -5697323553971165323}};
static struct { unsigned a:3; unsigned b:5; int c:24; } g2 = {6, 14, 2772654};
static unsigned char[16] g3 = { 130, 16, 158, 44, 186, 72, 214, 100, 242, 128, 14, 156, 42, 184, 70, 212 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
