#include <stdio.h>
extern int g0;
extern int f1(int);
extern int g2;
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-34));
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](8));
printf("tbl4=%d\\n", tbl4[1](16));
printf("mid5=%d\\n", mid5(11));

    return 0;
}
