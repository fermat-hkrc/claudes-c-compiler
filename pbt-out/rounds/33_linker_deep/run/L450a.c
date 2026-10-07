#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int f2(int);
extern int g3;
extern int mid4(int);
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](5));
printf("f2=%d\\n", f2(11));
printf("g3=%d\\n", g3);
printf("mid4=%d\\n", mid4(-42));
printf("g5=%d\\n", g5);

    return 0;
}
