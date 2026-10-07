#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-44));
printf("tbl2=%d\\n", tbl2[1](-16));
printf("tent3=%d\\n", init_tent3());
printf("mid4=%d\\n", mid4(-45));
printf("mid5=%d\\n", mid5(-7));

    return 0;
}
