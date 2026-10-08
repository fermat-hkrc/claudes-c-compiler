#include <stdio.h>
extern int g0;
extern int f1(int);
extern int g2;
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-35));
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](12));
printf("f4=%d\\n", f4(-5));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
