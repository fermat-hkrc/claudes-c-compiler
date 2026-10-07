#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int g4;
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](5));
printf("tbl2=%d\\n", tbl2[1](-17));
printf("tbl3=%d\\n", tbl3[1](-18));
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(7));

    return 0;
}
