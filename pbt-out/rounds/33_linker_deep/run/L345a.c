#include <stdio.h>
extern int g0;
extern int f1(int);
extern int f2(int);
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-26));
printf("f2=%d\\n", f2(-10));
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](0));
printf("f5=%d\\n", f5(7));

    return 0;
}
