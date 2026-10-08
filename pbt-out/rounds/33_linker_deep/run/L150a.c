#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](19));
printf("mid2=%d\\n", mid2(-50));
printf("tbl3=%d\\n", tbl3[1](5));
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](19));

    return 0;
}
