#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int f2(int);
extern int arr3[4];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-25));
printf("f2=%d\\n", f2(-37));
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("tbl4=%d\\n", tbl4[1](14));
printf("tbl5=%d\\n", tbl5[1](-10));

    return 0;
}
