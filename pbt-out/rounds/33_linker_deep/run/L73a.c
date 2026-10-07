#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-5));
printf("tent2=%d\\n", init_tent2());
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](6));
printf("mid5=%d\\n", mid5(-31));

    return 0;
}
