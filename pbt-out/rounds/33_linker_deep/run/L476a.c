#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int f2(int);
extern int mid3(int);
extern int f4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](16));
printf("f2=%d\\n", f2(-17));
printf("mid3=%d\\n", mid3(-58));
printf("f4=%d\\n", f4(-41));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
