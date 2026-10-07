#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int arr2[4];
extern int mid3(int);
extern int arr4[4];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-3));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("mid3=%d\\n", mid3(-7));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("tbl5=%d\\n", tbl5[1](-4));

    return 0;
}
