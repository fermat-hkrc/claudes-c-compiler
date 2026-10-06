#include <stdio.h>
#include <stddef.h>
struct __attribute__((packed)) P {
    char c;
    int i;
    long l;
    double d;
};
struct __attribute__((aligned(32))) A {
    char c;
};
struct B {
    char c;
    int __attribute__((aligned(8))) i;
};
struct C {
    char a, b, c, d;
    short __attribute__((packed)) e;
};
int main(void) {
    printf("%zu %zu %zu %zu\n", sizeof(struct P), offsetof(struct P, i), offsetof(struct P, l), offsetof(struct P, d));
    printf("%zu %zu\n", sizeof(struct A), sizeof(struct B));
    printf("%zu %zu\n", sizeof(struct C), offsetof(struct C, e));
    return 0;
}
