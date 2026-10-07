#include <stdio.h>
extern int g0;
extern int f1(int);
extern int f2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int arr4[4];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-31));
printf("f2=%d\\n", f2(-19));
printf("tbl3=%d\\n", tbl3[1](12));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("f5=%d\\n", f5(-28));

    return 0;
}
