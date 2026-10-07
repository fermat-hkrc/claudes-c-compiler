#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int g2;
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int arr4[4];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](2));
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](-4));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("tent5=%d\\n", init_tent5());

    return 0;
}
