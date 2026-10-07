#include <stdio.h>
extern int g0;
extern int g1;
extern int f2(int);
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("f2=%d\\n", f2(-11));
printf("tent3=%d\\n", init_tent3());
printf("tbl4=%d\\n", tbl4[1](-3));
printf("mid5=%d\\n", mid5(-33));

    return 0;
}
