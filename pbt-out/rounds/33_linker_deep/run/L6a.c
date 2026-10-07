#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int g2;
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](19));
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](9));
printf("f4=%d\\n", f4(9));
printf("f5=%d\\n", f5(-6));

    return 0;
}
