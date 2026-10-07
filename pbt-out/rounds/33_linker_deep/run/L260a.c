#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int f4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-33));
printf("tbl2=%d\\n", tbl2[1](-8));
printf("tent3=%d\\n", init_tent3());
printf("f4=%d\\n", f4(-31));
printf("mid5=%d\\n", mid5(-5));

    return 0;
}
