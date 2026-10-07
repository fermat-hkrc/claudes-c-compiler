#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int mid2(int);
extern int f3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-48));
printf("mid2=%d\\n", mid2(-35));
printf("f3=%d\\n", f3(-37));
printf("tbl4=%d\\n", tbl4[1](3));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
