#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-8));
printf("mid2=%d\\n", mid2(-12));
printf("mid3=%d\\n", mid3(-34));
printf("tbl4=%d\\n", tbl4[1](-9));
printf("f5=%d\\n", f5(-31));

    return 0;
}
