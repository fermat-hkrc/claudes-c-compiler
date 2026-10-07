#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(1));
printf("mid2=%d\\n", mid2(1));
printf("tbl3=%d\\n", tbl3[1](-18));
printf("tent4=%d\\n", init_tent4());
printf("g5=%d\\n", g5);

    return 0;
}
