#include <stdio.h>
#include <string.h>

static struct { char x[3]; int y; struct { short p; long q; } z; } g0 = {{'', '', ''}, 21501, {21501, -2751824628937530115}};
static long long g1 = 723528937u;
static int g2 = 434642;
static short[5] g3 = {8675, 75, 28675};

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    printf("g2=%llx %zu\n", (unsigned long long)g2, sizeof(g2));
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
