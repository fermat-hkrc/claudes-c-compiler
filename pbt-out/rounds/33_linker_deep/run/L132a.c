#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int g3;
extern int mid4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(7));
printf("tbl2=%d\\n", tbl2[1](2));
printf("g3=%d\\n", g3);
printf("mid4=%d\\n", mid4(1));
printf("f5=%d\\n", f5(-30));

    return 0;
}
