#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int f2(int);
extern int mid3(int);
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](3));
printf("f2=%d\\n", f2(-2));
printf("mid3=%d\\n", mid3(-24));
printf("mid4=%d\\n", mid4(-28));
printf("mid5=%d\\n", mid5(-42));

    return 0;
}
