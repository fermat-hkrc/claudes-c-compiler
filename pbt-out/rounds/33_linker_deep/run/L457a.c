#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int g2;
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("g2=%d\\n", g2);
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](-3));
printf("f5=%d\\n", f5(1));

    return 0;
}
