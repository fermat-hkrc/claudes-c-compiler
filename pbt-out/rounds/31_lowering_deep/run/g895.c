#include <stdio.h>
#include <string.h>

static short[5] g0 = {28276, 76, 28276};
static long long g1 = 2855813790u;
static struct { int a; char b; double c; } g2 = {49758, '{', 4523.454545454545};
static struct { char x[3]; int y; struct { short p; long q; } z; } g3 = {{'x', 'y', 'z'}, 22989, {2989, 1970101422681122989}};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
