#include <stdio.h>
extern long mut_a_impl(long, int);
extern long shared_arr[16];
int main(void) {
    long r = mut_a_impl(1, 0);
    long s = 0;
    for (int i = 0; i < 16; i++) s += shared_arr[i];
    printf("%ld %ld\n", r, s);
    return 0;
}
