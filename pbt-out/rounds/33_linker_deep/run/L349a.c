#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("mid2=%d\\n", mid2(-44));
printf("tbl3=%d\\n", tbl3[1](-15));
printf("f4=%d\\n", f4(-6));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
