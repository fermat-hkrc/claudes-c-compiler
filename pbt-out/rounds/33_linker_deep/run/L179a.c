#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int g2;
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](-15));
printf("tbl4=%d\\n", tbl4[1](16));
printf("g5=%d\\n", g5);

    return 0;
}
