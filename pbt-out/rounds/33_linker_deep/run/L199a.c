#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int arr2[4];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-23));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("tbl3=%d\\n", tbl3[1](18));
printf("f4=%d\\n", f4(-2));
printf("g5=%d\\n", g5);

    return 0;
}
