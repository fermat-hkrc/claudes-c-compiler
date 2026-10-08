#include <stdio.h>
extern int g0;
extern int g1;
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tbl2=%d\\n", tbl2[1](0));
printf("tent3=%d\\n", init_tent3());
printf("f4=%d\\n", f4(-36));
printf("tbl5=%d\\n", tbl5[1](8));

    return 0;
}
