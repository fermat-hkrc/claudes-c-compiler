#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int f3(int);
extern int f4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-9));
printf("tbl2=%d\\n", tbl2[1](9));
printf("f3=%d\\n", f3(-31));
printf("f4=%d\\n", f4(-43));
printf("f5=%d\\n", f5(9));

    return 0;
}
