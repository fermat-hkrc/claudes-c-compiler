#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int g2;
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-1));
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(9));
printf("tbl4=%d\\n", tbl4[1](-3));
printf("f5=%d\\n", f5(-31));

    return 0;
}
