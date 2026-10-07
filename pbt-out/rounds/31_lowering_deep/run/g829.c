#include <stdio.h>
#include <string.h>

static int[4][3] g0 = {{17, 1017, 17}, {3017, 1017}};
static struct { char x[3]; int y; struct { short p; long q; } z; } g1 = {{'{', '|', '}'}, 35286, {15286, -1675714866355416330}};
static int[4][3] g2 = {{73, 1073, 1073}, {3073, 1073}};
static char[8] g3 = {'e', 'E', '|', 80};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
