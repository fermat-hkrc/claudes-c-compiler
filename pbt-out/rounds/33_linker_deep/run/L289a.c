#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int arr2[4];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-14));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("tbl3=%d\\n", tbl3[1](-9));
printf("f4=%d\\n", f4(-43));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
