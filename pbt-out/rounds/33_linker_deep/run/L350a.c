#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int g3;
extern int mid4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-17));
printf("tbl2=%d\\n", tbl2[1](16));
printf("g3=%d\\n", g3);
printf("mid4=%d\\n", mid4(2));
printf("f5=%d\\n", f5(-7));

    return 0;
}
