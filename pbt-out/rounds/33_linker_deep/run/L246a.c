#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int f4(int);
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-1));
printf("tbl2=%d\\n", tbl2[1](-2));
printf("mid3=%d\\n", mid3(-10));
printf("f4=%d\\n", f4(-43));
printf("g5=%d\\n", g5);

    return 0;
}
