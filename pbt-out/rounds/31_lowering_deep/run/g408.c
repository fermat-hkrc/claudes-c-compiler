#include <stdio.h>
#include <string.h>

static struct { int a; char b; double c; } g0 = {92040, 'w', 8367.272727272728};
static long long g1 = 426786972u;
static long long g2 = 3419903425u;
static unsigned char[16] g3 = { 134, 48, 218, 132, 46, 216, 130, 44, 214, 128, 42, 212, 126, 40, 210, 124 };

int main(void) {
    { unsigned char b[512]; unsigned n = sizeof(g0); memcpy(b, &g0, n); printf("g0=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    printf("g1=%llx %zu\n", (unsigned long long)g1, sizeof(g1));
    printf("g2=%llx %zu\n", (unsigned long long)g2, sizeof(g2));
    { unsigned char b[512]; unsigned n = sizeof(g3); memcpy(b, &g3, n); printf("g3=%zu:", n); for (unsigned j = 0; j < n; j++) printf("%02x", b[j]); putchar('\n'); }
    return 0;
}
