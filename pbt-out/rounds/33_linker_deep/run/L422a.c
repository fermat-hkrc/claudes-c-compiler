#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int g4;
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tbl2=%d\\n", tbl2[1](13));
printf("mid3=%d\\n", mid3(18));
printf("g4=%d\\n", g4);
printf("f5=%d\\n", f5(-9));

    return 0;
}
