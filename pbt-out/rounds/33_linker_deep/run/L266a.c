#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int arr2[4];
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-39));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](4));
printf("tbl5=%d\\n", tbl5[1](-17));

    return 0;
}
