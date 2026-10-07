#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int arr4[4];
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tent2=%d\\n", init_tent2());
printf("tbl3=%d\\n", tbl3[1](-13));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("g5=%d\\n", g5);

    return 0;
}
