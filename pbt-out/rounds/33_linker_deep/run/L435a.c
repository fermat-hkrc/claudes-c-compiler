#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int f2(int);
extern int mid3(int);
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-22));
printf("f2=%d\\n", f2(-13));
printf("mid3=%d\\n", mid3(15));
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](12));

    return 0;
}
