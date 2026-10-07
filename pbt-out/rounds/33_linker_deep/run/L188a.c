#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int f2(int);
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-1));
printf("f2=%d\\n", f2(-11));
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](9));
printf("f5=%d\\n", f5(-20));

    return 0;
}
