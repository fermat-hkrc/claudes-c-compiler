#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int arr3[4];
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-19));
printf("mid2=%d\\n", mid2(14));
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("f4=%d\\n", f4(-6));
printf("tbl5=%d\\n", tbl5[1](5));

    return 0;
}
