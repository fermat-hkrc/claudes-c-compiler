#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-41));
printf("tbl2=%d\\n", tbl2[1](9));
printf("tbl3=%d\\n", tbl3[1](-8));
printf("tbl4=%d\\n", tbl4[1](16));
printf("g5=%d\\n", g5);

    return 0;
}
