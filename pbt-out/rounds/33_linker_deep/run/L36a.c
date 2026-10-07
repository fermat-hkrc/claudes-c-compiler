#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-28));
printf("tbl2=%d\\n", tbl2[1](-10));
printf("mid3=%d\\n", mid3(-53));
printf("tbl4=%d\\n", tbl4[1](1));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
