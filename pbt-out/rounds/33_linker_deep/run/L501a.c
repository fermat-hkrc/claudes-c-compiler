#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int g3;
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-14));
printf("mid2=%d\\n", mid2(8));
printf("g3=%d\\n", g3);
printf("mid4=%d\\n", mid4(-59));
printf("tbl5=%d\\n", tbl5[1](12));

    return 0;
}
