#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-20));
printf("tbl2=%d\\n", tbl2[1](-14));
printf("tbl3=%d\\n", tbl3[1](-19));
printf("tbl4=%d\\n", tbl4[1](9));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
