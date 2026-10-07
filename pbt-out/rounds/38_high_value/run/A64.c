#include <stdio.h>
#include <string.h>
struct SD65 { double m0; float m1; };

static struct SD65 mk(void) {
    struct SD65 s;
    unsigned char *p = (unsigned char*)&s;
    for (unsigned i = 0; i < sizeof(s); i++) p[i] = (unsigned char)(4140923065u >> (i % 64)) + i;
    return s;
}
static unsigned long long take(struct SD65 s) {
    unsigned long long h = 1469598103934665603ULL;
    unsigned char *p = (unsigned char*)&s;
    for (unsigned i = 0; i < sizeof(s); i++) h = (h ^ p[i]) * 1099511628211ULL;
    return h;
}
int main(void) {
    struct SD65 a = mk(); struct SD65 b = mk();
    printf("take=%llu ret-hash=", take(a));
    struct SD65 c = mk();
    unsigned char *p = (unsigned char*)&c;
    unsigned long long h = 1469598103934665603ULL;
    for (unsigned i = 0; i < sizeof(c); i++) h = (h ^ p[i]) * 1099511628211ULL;
    printf("%llu sizeof=%zu\n", h, sizeof(struct SD65));
    return 0;
}