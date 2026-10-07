#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int mid2(int);
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("mid2=%d\\n", mid2(-57));
printf("mid3=%d\\n", mid3(-33));
printf("tbl4=%d\\n", tbl4[1](10));
printf("g5=%d\\n", g5);

    return 0;
}
