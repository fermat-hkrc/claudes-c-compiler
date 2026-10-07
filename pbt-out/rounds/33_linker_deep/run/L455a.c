#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("mid2=%d\\n", mid2(-40));
printf("tbl3=%d\\n", tbl3[1](-17));
printf("tbl4=%d\\n", tbl4[1](-2));
printf("mid5=%d\\n", mid5(-39));

    return 0;
}
