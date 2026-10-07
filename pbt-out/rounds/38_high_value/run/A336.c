#include <stdio.h>
#include <string.h>
struct SD1 { short m0; long m1; };

static struct SD1 mk(void) {
    struct SD1 s;
    unsigned char *p = (unsigned char*)&s;
    for (unsigned i = 0; i < sizeof(s); i++) p[i] = (unsigned char)(17785129142037700013u >> (i % 64)) + i;
    return s;
}
static unsigned long long take(struct SD1 s) {
    unsigned long long h = 1469598103934665603ULL;
    unsigned char *p = (unsigned char*)&s;
    for (unsigned i = 0; i < sizeof(s); i++) h = (h ^ p[i]) * 1099511628211ULL;
    return h;
}
int main(void) {
    struct SD1 a = mk(); struct SD1 b = mk();
    printf("take=%llu ret-hash=", take(a));
    struct SD1 c = mk();
    unsigned char *p = (unsigned char*)&c;
    unsigned long long h = 1469598103934665603ULL;
    for (unsigned i = 0; i < sizeof(c); i++) h = (h ^ p[i]) * 1099511628211ULL;
    printf("%llu sizeof=%zu\n", h, sizeof(struct SD1));
    return 0;
}