#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int f3(int);
extern int mid4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-26));
printf("tbl2=%d\\n", tbl2[1](13));
printf("f3=%d\\n", f3(6));
printf("tent4=%d\\n", init_tent4());
printf("f5=%d\\n", f5(-31));

    return 0;
}
