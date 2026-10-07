#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int arr2[4];
extern int arr3[4];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](14));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("tbl4=%d\\n", tbl4[1](-3));
printf("mid5=%d\\n", mid5(-39));

    return 0;
}
