#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int arr4[4];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-40));
printf("tent2=%d\\n", init_tent2());
printf("tbl3=%d\\n", tbl3[1](8));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("tent5=%d\\n", init_tent5());

    return 0;
}
