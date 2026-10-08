#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int g4;
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-34));
printf("mid2=%d\\n", mid2(-50));
printf("tbl3=%d\\n", tbl3[1](-11));
printf("g4=%d\\n", g4);
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
