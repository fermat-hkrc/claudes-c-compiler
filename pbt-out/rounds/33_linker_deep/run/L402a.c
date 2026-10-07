#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int f3(int);
extern int mid4(int);
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-25));
printf("tbl2=%d\\n", tbl2[1](19));
printf("f3=%d\\n", f3(-37));
printf("mid4=%d\\n", mid4(-32));
printf("g5=%d\\n", g5);

    return 0;
}
