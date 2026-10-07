#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-13));
printf("tbl2=%d\\n", tbl2[1](-6));
printf("tent3=%d\\n", init_tent3());
printf("tbl4=%d\\n", tbl4[1](-14));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
