#include <stdio.h>
int foo(int x) { return x; }
typedef int (*fp_t)(int);
int main(void) {
    fp_t a = foo, b = foo;
    fp_t arr[2] = { foo, foo };
    printf("%d %d %d\n", a == b, arr[0] == arr[1], a == foo);
    return 0;
}
