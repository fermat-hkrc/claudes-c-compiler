#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-20));
printf("tbl2=%d\\n", tbl2[1](-18));
printf("mid3=%d\\n", mid3(-8));
printf("f4=%d\\n", f4(-23));
printf("tbl5=%d\\n", tbl5[1](8));

    return 0;
}
