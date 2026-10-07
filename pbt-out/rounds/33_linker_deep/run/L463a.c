#include <stdio.h>
extern int g0;
extern int g1;
extern int g2;
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](-2));
printf("f4=%d\\n", f4(-23));
printf("f5=%d\\n", f5(3));

    return 0;
}
