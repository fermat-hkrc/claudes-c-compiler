#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int mid2(int);
extern int arr3[4];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tent2=%d\\n", init_tent2());
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("tbl4=%d\\n", tbl4[1](6));
printf("f5=%d\\n", f5(-25));

    return 0;
}
