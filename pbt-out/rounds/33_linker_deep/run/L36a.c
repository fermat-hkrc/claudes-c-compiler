#include <stdio.h>
extern int g0;
extern int f1(int);
extern int f2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-34));
printf("f2=%d\\n", f2(-31));
printf("tbl3=%d\\n", tbl3[1](17));
printf("f4=%d\\n", f4(-12));
printf("tbl5=%d\\n", tbl5[1](-15));

    return 0;
}
