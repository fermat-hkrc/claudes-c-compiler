#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
extern int f3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-2));
printf("tent2=%d\\n", init_tent2());
printf("f3=%d\\n", f3(10));
printf("tbl4=%d\\n", tbl4[1](2));
printf("f5=%d\\n", f5(-18));

    return 0;
}
