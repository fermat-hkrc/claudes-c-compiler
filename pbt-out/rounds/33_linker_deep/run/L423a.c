#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int f3(int);
extern int mid4(int);
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-57));
printf("tbl2=%d\\n", tbl2[1](1));
printf("f3=%d\\n", f3(-7));
printf("mid4=%d\\n", mid4(-34));
printf("g5=%d\\n", g5);

    return 0;
}
