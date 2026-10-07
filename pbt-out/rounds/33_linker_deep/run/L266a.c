#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](6));
printf("tent2=%d\\n", init_tent2());
printf("tbl3=%d\\n", tbl3[1](-17));
printf("tbl4=%d\\n", tbl4[1](9));
printf("tbl5=%d\\n", tbl5[1](0));

    return 0;
}
