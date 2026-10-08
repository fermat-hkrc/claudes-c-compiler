#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(10));
printf("tent2=%d\\n", init_tent2());
printf("tbl3=%d\\n", tbl3[1](7));
printf("mid4=%d\\n", mid4(-11));
printf("tbl5=%d\\n", tbl5[1](6));

    return 0;
}
