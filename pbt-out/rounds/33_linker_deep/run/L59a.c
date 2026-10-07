#include <stdio.h>
extern int g0;
extern int arr1[4];
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tbl2=%d\\n", tbl2[1](15));
printf("tbl3=%d\\n", tbl3[1](19));
printf("mid4=%d\\n", mid4(-7));
printf("tbl5=%d\\n", tbl5[1](-10));

    return 0;
}
