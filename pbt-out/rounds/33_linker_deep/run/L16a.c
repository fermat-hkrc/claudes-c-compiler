#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int f4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tbl2=%d\\n", tbl2[1](3));
printf("mid3=%d\\n", mid3(-1));
printf("f4=%d\\n", f4(13));
printf("f5=%d\\n", f5(-42));

    return 0;
}
