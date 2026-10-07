#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-5));
printf("mid2=%d\\n", mid2(-37));
printf("tbl3=%d\\n", tbl3[1](-13));
printf("tbl4=%d\\n", tbl4[1](14));
printf("g5=%d\\n", g5);

    return 0;
}
