#include <stdio.h>
extern int g0;
extern int g1;
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int g3;
extern int mid4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tbl2=%d\\n", tbl2[1](1));
printf("g3=%d\\n", g3);
printf("tent4=%d\\n", init_tent4());
printf("f5=%d\\n", f5(-9));

    return 0;
}
