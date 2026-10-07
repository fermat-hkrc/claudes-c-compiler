#include <stdio.h>
extern int g0;
extern int f1(int);
extern int f2(int);
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-13));
printf("f2=%d\\n", f2(7));
printf("mid3=%d\\n", mid3(-12));
printf("tbl4=%d\\n", tbl4[1](14));
printf("g5=%d\\n", g5);

    return 0;
}
