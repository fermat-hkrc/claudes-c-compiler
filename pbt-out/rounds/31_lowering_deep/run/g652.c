#include <stdio.h>
#include <string.h>

static struct { int a; char b; double c; } g0 = {56665, '~', 5151.363636363636};
static struct { char x[3]; int y; struct { short p; long q; } z; } g1 = {{'', '', ''}, 71258, {1258, -6001817959858580358}};
static int g2 = 54166;
static int[4][3] g3 = {{625, 625, 1625}, {2625, 625}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g2=%llx %zu\n", (unsigned long long)g2, sizeof(g2));
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
