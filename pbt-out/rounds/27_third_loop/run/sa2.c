#include <stdio.h>
struct Aligned { _Alignas(16) char c; };
int main(void) {
    _Static_assert(sizeof(struct Aligned) == 16, "aligned size");
    printf("%zu %zu\n", sizeof(struct Aligned), _Alignof(struct Aligned));
    return 0;
}
