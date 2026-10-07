#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int g3;
extern int f4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tbl2=%d\\n", tbl2[1](-2));
printf("g3=%d\\n", g3);
printf("f4=%d\\n", f4(-26));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
