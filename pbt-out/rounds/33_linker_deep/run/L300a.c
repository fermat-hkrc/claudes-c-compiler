#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int f2(int);
extern int g3;
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-17));
printf("f2=%d\\n", f2(14));
printf("g3=%d\\n", g3);
printf("mid4=%d\\n", mid4(-31));
printf("tbl5=%d\\n", tbl5[1](7));

    return 0;
}
