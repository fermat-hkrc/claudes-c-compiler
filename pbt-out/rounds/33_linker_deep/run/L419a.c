#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int g2;
extern int g3;
extern int f4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-12));
printf("g2=%d\\n", g2);
printf("g3=%d\\n", g3);
printf("f4=%d\\n", f4(-26));
printf("f5=%d\\n", f5(-23));

    return 0;
}
