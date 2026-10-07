#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int f2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-25));
printf("f2=%d\\n", f2(10));
printf("tbl3=%d\\n", tbl3[1](-8));
printf("mid4=%d\\n", mid4(15));
printf("f5=%d\\n", f5(8));

    return 0;
}
