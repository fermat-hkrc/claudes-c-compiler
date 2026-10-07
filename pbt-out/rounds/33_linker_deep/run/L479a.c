#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int g2;
extern int f3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-23));
printf("g2=%d\\n", g2);
printf("f3=%d\\n", f3(-21));
printf("tbl4=%d\\n", tbl4[1](4));
printf("g5=%d\\n", g5);

    return 0;
}
