#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](2));
printf("tbl2=%d\\n", tbl2[1](8));
printf("mid3=%d\\n", mid3(-14));
printf("tbl4=%d\\n", tbl4[1](-2));
printf("f5=%d\\n", f5(-44));

    return 0;
}
