#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tbl2=%d\\n", tbl2[1](14));
printf("tbl3=%d\\n", tbl3[1](7));
printf("f4=%d\\n", f4(-23));
printf("mid5=%d\\n", mid5(13));

    return 0;
}
