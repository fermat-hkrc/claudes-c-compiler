#include <stdio.h>
extern int g0;
extern int g1;
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("mid2=%d\\n", mid2(2));
printf("tbl3=%d\\n", tbl3[1](-19));
printf("tbl4=%d\\n", tbl4[1](3));
printf("tbl5=%d\\n", tbl5[1](-9));

    return 0;
}
