#include <stdio.h>
extern int g0;
extern int f1(int);
extern int g2;
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(5));
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](15));
printf("f4=%d\\n", f4(-12));
printf("tbl5=%d\\n", tbl5[1](10));

    return 0;
}
