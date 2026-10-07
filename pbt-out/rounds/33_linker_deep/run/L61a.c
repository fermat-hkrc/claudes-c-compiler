#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-39));
printf("tbl2=%d\\n", tbl2[1](19));
printf("mid3=%d\\n", mid3(-45));
printf("tbl4=%d\\n", tbl4[1](-18));
printf("g5=%d\\n", g5);

    return 0;
}
