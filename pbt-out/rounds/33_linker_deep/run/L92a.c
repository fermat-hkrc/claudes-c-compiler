#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int g2;
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-37));
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](16));
printf("mid4=%d\\n", mid4(-37));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
