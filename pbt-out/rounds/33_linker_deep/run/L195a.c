#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int f2(int);
extern int f3(int);
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-2));
printf("f2=%d\\n", f2(-7));
printf("f3=%d\\n", f3(-14));
printf("mid4=%d\\n", mid4(-18));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
