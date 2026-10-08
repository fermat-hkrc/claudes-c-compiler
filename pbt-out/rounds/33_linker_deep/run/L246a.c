#include <stdio.h>
extern int g0;
extern int g1;
extern int mid2(int);
extern int f3(int);
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tent2=%d\\n", init_tent2());
printf("f3=%d\\n", f3(1));
printf("mid4=%d\\n", mid4(-45));
printf("tbl5=%d\\n", tbl5[1](1));

    return 0;
}
