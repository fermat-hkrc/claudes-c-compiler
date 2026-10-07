#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-48));
printf("tbl2=%d\\n", tbl2[1](-17));
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](14));
printf("f5=%d\\n", f5(2));

    return 0;
}
