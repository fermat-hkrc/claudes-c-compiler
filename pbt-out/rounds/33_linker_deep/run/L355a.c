#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(6));
printf("tent2=%d\\n", init_tent2());
printf("tbl3=%d\\n", tbl3[1](-12));
printf("f4=%d\\n", f4(6));
printf("f5=%d\\n", f5(-1));

    return 0;
}
