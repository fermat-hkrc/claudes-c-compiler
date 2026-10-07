#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int g4;
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-23));
printf("mid2=%d\\n", mid2(-59));
printf("tbl3=%d\\n", tbl3[1](-5));
printf("g4=%d\\n", g4);
printf("tbl5=%d\\n", tbl5[1](17));

    return 0;
}
