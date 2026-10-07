#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-21));
printf("tbl2=%d\\n", tbl2[1](18));
printf("tbl3=%d\\n", tbl3[1](16));
printf("tent4=%d\\n", init_tent4());
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
