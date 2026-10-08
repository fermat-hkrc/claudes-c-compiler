#include <stdio.h>
extern int g0;
extern int g1;
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("mid2=%d\\n", mid2(-15));
printf("tbl3=%d\\n", tbl3[1](-7));
printf("f4=%d\\n", f4(-25));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
