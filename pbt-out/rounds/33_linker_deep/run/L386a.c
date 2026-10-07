#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-17));
printf("mid2=%d\\n", mid2(-47));
printf("tent3=%d\\n", init_tent3());
printf("tbl4=%d\\n", tbl4[1](14));
printf("tbl5=%d\\n", tbl5[1](-18));

    return 0;
}
