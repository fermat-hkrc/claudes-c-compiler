#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int f3(int);
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-10));
printf("tbl2=%d\\n", tbl2[1](-6));
printf("f3=%d\\n", f3(10));
printf("mid4=%d\\n", mid4(-23));
printf("mid5=%d\\n", mid5(-13));

    return 0;
}
