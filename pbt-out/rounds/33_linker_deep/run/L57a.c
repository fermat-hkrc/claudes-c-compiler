#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tbl2=%d\\n", tbl2[1](-5));
printf("mid3=%d\\n", mid3(-54));
printf("mid4=%d\\n", mid4(-12));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
