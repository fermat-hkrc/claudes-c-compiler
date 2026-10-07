#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
extern int mid3(int);
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(4));
printf("mid2=%d\\n", mid2(6));
printf("mid3=%d\\n", mid3(-17));
printf("mid4=%d\\n", mid4(-9));
printf("tbl5=%d\\n", tbl5[1](-8));

    return 0;
}
