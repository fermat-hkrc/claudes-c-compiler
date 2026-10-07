#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int f2(int);
extern int g3;
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("f2=%d\\n", f2(13));
printf("g3=%d\\n", g3);
printf("mid4=%d\\n", mid4(-42));
printf("tbl5=%d\\n", tbl5[1](-11));

    return 0;
}
