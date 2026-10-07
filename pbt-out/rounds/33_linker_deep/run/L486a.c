#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-17));
printf("mid2=%d\\n", mid2(18));
printf("tbl3=%d\\n", tbl3[1](14));
printf("mid4=%d\\n", mid4(3));
printf("mid5=%d\\n", mid5(8));

    return 0;
}
