#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int f3(int);
extern int mid4(int);
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](4));
printf("tent2=%d\\n", init_tent2());
printf("f3=%d\\n", f3(-17));
printf("mid4=%d\\n", mid4(-19));
printf("g5=%d\\n", g5);

    return 0;
}
