#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int mid3(int);
extern int mid4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-13));
printf("tent2=%d\\n", init_tent2());
printf("mid3=%d\\n", mid3(-52));
printf("mid4=%d\\n", mid4(-13));
printf("f5=%d\\n", f5(-23));

    return 0;
}
