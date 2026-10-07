#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-4));
printf("tent2=%d\\n", init_tent2());
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](2));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
