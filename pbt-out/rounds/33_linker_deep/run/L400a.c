#include <stdio.h>
extern int g0;
extern int f1(int);
extern int g2;
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-33));
printf("g2=%d\\n", g2);
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](9));
printf("f5=%d\\n", f5(9));

    return 0;
}
