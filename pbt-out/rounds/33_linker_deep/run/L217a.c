#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int g2;
extern int f3(int);
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](18));
printf("g2=%d\\n", g2);
printf("f3=%d\\n", f3(-17));
printf("f4=%d\\n", f4(-15));
printf("tbl5=%d\\n", tbl5[1](-14));

    return 0;
}
