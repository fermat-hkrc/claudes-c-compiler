#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int g2;
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](14));
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](17));
printf("f4=%d\\n", f4(-12));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
