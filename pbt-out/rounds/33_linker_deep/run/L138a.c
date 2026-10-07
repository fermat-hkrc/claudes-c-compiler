#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int mid2(int);
extern int f3(int);
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("mid2=%d\\n", mid2(-36));
printf("f3=%d\\n", f3(13));
printf("f4=%d\\n", f4(-36));
printf("tbl5=%d\\n", tbl5[1](5));

    return 0;
}
