#include <stdio.h>
#include <string.h>

static int g0 = -258047;
static struct { char x[3]; int y; struct { short p; long q; } z; } g1 = {{'', '', ''}, 28321, {8321, -2690931230507623295}};
static int[4][3] g2 = {{588, 1588, 1588}, {3588, 1588}};
static struct { int a; char b; double c; } g3 = {41926, '', 3811.4545454545455};

int main(void) {
    printf("g0=%llx %zu\n", (unsigned long long)g0, sizeof(g0));
    { unsigned char b[512]; unsigned n = sizeof(g1); memcpy(b, &g1, n); printf("g1=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g2); memcpy(b, &g2, n); printf("g2=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
