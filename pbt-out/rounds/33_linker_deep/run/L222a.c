#include <stdio.h>
extern int g0;
extern int g1;
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tent2=%d\\n", init_tent2());
printf("tbl3=%d\\n", tbl3[1](11));
printf("tbl4=%d\\n", tbl4[1](-2));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
