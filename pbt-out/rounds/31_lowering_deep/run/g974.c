#include <stdio.h>
#include <string.h>

static int[4][3] g0 = {{554, 1554, 554}, {3554, 1554}};
static int[4][3] g1 = {{944, 1944, 2944}, {1944, 3944}};
static struct { unsigned x:1; unsigned y:31; } g2 = {1, 7378143760388251001};
static struct { char x[3]; int y; struct { short p; long q; } z; } g3 = {{'{', '|', '}'}, 87086, {17086, 524317447381787086}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
