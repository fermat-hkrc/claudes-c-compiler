#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int f3(int);
extern int g4;
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tbl2=%d\\n", tbl2[1](14));
printf("f3=%d\\n", f3(12));
printf("g4=%d\\n", g4);
printf("g5=%d\\n", g5);

    return 0;
}
