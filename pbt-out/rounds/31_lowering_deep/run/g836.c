#include <stdio.h>
#include <string.h>

static int[4][3] g0 = {{270, 1270, 270}, {3270, 1270}};
static struct { char x[3]; int y; struct { short p; long q; } z; } g1 = {{'', '', ''}, 13628, {3628, 958193685645813628}};
static int[4][3] g2 = {{62, 1062, 1062}, {3062, 4062}};
static int g3 = 333954;

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g3=%llx %zu\n", (unsigned long long)g3, sizeof(g3));
    return 0;
}
