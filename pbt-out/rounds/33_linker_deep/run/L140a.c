#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-22));
printf("tbl2=%d\\n", tbl2[1](-8));
printf("tbl3=%d\\n", tbl3[1](-18));
printf("f4=%d\\n", f4(-34));
printf("g5=%d\\n", g5);

    return 0;
}
