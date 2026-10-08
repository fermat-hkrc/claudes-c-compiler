#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-11));
printf("tbl2=%d\\n", tbl2[1](1));
printf("tent3=%d\\n", init_tent3());
printf("mid4=%d\\n", mid4(-14));
printf("tbl5=%d\\n", tbl5[1](14));

    return 0;
}
