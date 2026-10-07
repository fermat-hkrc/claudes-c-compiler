#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int f2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int g4;
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-54));
printf("f2=%d\\n", f2(-3));
printf("tbl3=%d\\n", tbl3[1](-9));
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(4));

    return 0;
}
