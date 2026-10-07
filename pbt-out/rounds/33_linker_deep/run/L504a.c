#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int g4;
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-4));
printf("tbl2=%d\\n", tbl2[1](6));
printf("tbl3=%d\\n", tbl3[1](13));
printf("g4=%d\\n", g4);
printf("f5=%d\\n", f5(-44));

    return 0;
}
