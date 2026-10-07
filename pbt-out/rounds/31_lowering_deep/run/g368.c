#include <stdio.h>
#include <string.h>

static long long g0 = 3731512637u;
static int[4][3] g1 = {{127, 1127, 1127}, {1127, 127}};
static struct { char x[3]; int y; struct { short p; long q; } z; } g2 = {{'', '', ''}, 17547, {7547, -5777703064700834069}};
static unsigned char[16] g3 = { 3, 152, 45, 194, 87, 236, 129, 22, 171, 64, 213, 106, 255, 148, 41, 190 };

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
