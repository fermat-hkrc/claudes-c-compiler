#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-42));
printf("tbl2=%d\\n", tbl2[1](6));
printf("tbl3=%d\\n", tbl3[1](-4));
printf("f4=%d\\n", f4(-42));
printf("tbl5=%d\\n", tbl5[1](5));

    return 0;
}
