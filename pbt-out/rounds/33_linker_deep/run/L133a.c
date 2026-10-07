#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int f2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(2));
printf("f2=%d\\n", f2(-30));
printf("tbl3=%d\\n", tbl3[1](-8));
printf("tbl4=%d\\n", tbl4[1](-18));
printf("g5=%d\\n", g5);

    return 0;
}
