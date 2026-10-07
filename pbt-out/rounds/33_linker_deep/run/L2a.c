#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
extern int f3(int);
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(9));
printf("mid2=%d\\n", mid2(-12));
printf("f3=%d\\n", f3(-15));
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](18));

    return 0;
}
