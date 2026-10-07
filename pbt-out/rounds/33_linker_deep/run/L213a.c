#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("mid2=%d\\n", mid2(-41));
printf("tbl3=%d\\n", tbl3[1](-1));
printf("f4=%d\\n", f4(-12));
printf("tbl5=%d\\n", tbl5[1](-18));

    return 0;
}
