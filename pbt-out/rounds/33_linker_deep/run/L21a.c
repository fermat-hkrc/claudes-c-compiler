#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int f2(int);
extern int f3(int);
extern int f4(int);
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-18));
printf("f2=%d\\n", f2(-10));
printf("f3=%d\\n", f3(-1));
printf("f4=%d\\n", f4(-12));
printf("g5=%d\\n", g5);

    return 0;
}
