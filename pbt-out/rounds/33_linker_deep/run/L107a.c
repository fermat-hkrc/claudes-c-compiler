#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int g3;
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-33));
printf("tbl2=%d\\n", tbl2[1](3));
printf("g3=%d\\n", g3);
printf("f4=%d\\n", f4(-7));
printf("tbl5=%d\\n", tbl5[1](15));

    return 0;
}
