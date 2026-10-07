#include <stdio.h>
extern int g0;
extern int f1(int);
extern int f2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-26));
printf("f2=%d\\n", f2(1));
printf("tbl3=%d\\n", tbl3[1](-4));
printf("tent4=%d\\n", init_tent4());
printf("tent5=%d\\n", init_tent5());

    return 0;
}
