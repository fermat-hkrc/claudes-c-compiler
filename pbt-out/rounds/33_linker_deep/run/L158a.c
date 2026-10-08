#include <stdio.h>
extern int g0;
extern int f1(int);
extern int f2(int);
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-27));
printf("f2=%d\\n", f2(-43));
printf("tent3=%d\\n", init_tent3());
printf("tbl4=%d\\n", tbl4[1](1));
printf("tbl5=%d\\n", tbl5[1](-9));

    return 0;
}
