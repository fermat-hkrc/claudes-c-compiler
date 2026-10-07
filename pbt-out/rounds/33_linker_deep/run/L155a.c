#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int f2(int);
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-45));
printf("f2=%d\\n", f2(-31));
printf("mid3=%d\\n", mid3(-20));
printf("tbl4=%d\\n", tbl4[1](9));
printf("f5=%d\\n", f5(-9));

    return 0;
}
