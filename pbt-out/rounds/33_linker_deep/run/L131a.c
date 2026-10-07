#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int f4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(14));
printf("tbl2=%d\\n", tbl2[1](-6));
printf("tent3=%d\\n", init_tent3());
printf("f4=%d\\n", f4(-25));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
