#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int arr2[4];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("tbl3=%d\\n", tbl3[1](-17));
printf("tbl4=%d\\n", tbl4[1](-10));
printf("g5=%d\\n", g5);

    return 0;
}
