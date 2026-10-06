#include <stdio.h>
#include <stdarg.h>
long mixed(int n, ...) {
    va_list ap;
    va_start(ap, n);
    long total = 0;
    for (int i = 0; i < n; i++) {
        switch (i % 4) {
            case 0: total += va_arg(ap, int); break;
            case 1: total += (long)va_arg(ap, double); break;
            case 2: total += (long)va_arg(ap, long long); break;
            case 3: total += *va_arg(ap, char*); break;
        }
    }
    va_end(ap);
    return total;
}
int main(void) {
    char c = 'A';
    printf("%ld %ld\n",
        mixed(4, 10, 2.5, 100LL, &c),
        mixed(8, 1, 3.0, 200LL, &c, 5, 6.0, 300LL, &c));
    return 0;
}
