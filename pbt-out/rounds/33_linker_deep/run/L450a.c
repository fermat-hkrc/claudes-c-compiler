#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int f3(int);
extern int g4;
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-3));
printf("mid2=%d\\n", mid2(-10));
printf("f3=%d\\n", f3(-29));
printf("g4=%d\\n", g4);
printf("f5=%d\\n", f5(8));

    return 0;
}
