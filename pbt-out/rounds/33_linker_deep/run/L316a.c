#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](6));
printf("mid2=%d\\n", mid2(-3));
printf("mid3=%d\\n", mid3(0));
printf("tbl4=%d\\n", tbl4[1](-7));
printf("g5=%d\\n", g5);

    return 0;
}
