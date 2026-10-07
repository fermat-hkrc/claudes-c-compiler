#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(13));
printf("tbl2=%d\\n", tbl2[1](-13));
printf("tbl3=%d\\n", tbl3[1](-8));
printf("tent4=%d\\n", init_tent4());
printf("tent5=%d\\n", init_tent5());

    return 0;
}
