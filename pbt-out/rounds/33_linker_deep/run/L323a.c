#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(17));
printf("mid2=%d\\n", mid2(-20));
printf("tbl3=%d\\n", tbl3[1](-15));
printf("mid4=%d\\n", mid4(8));
printf("g5=%d\\n", g5);

    return 0;
}
