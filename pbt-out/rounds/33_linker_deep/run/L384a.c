#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](3));
printf("mid2=%d\\n", mid2(-13));
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](18));
printf("tbl5=%d\\n", tbl5[1](14));

    return 0;
}
