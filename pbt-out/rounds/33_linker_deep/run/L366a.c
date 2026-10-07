#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int f2(int);
extern int mid3(int);
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-11));
printf("f2=%d\\n", f2(-42));
printf("mid3=%d\\n", mid3(5));
printf("f4=%d\\n", f4(-9));
printf("tbl5=%d\\n", tbl5[1](-10));

    return 0;
}
