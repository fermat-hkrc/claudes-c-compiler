#include <stdio.h>
extern int g0;
extern int g1;
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tbl2=%d\\n", tbl2[1](-19));
printf("tent3=%d\\n", init_tent3());
printf("tbl4=%d\\n", tbl4[1](-4));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
