#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int f2(int);
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("f2=%d\\n", f2(-3));
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](-4));
printf("mid5=%d\\n", mid5(-22));

    return 0;
}
