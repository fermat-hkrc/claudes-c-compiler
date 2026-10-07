#include <stdio.h>
_Static_assert(sizeof(int) == 4, "int must be 4 bytes");
_Static_assert(sizeof(long) == 8, "long must be 8 bytes on LP64");
struct _Alignas(16) Aligned { char c; };
int main(void) {
    _Static_assert(sizeof(struct Aligned) == 16, "aligned size");
    printf("%zu %zu\n", sizeof(struct Aligned), _Alignof(struct Aligned));
    return 0;
}
