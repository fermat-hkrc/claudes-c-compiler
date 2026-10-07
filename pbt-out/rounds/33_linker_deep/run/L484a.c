#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int f3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](15));
printf("mid2=%d\\n", mid2(-20));
printf("f3=%d\\n", f3(-6));
printf("tbl4=%d\\n", tbl4[1](-4));
printf("f5=%d\\n", f5(-12));

    return 0;
}
