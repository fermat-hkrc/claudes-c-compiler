#include <stdio.h>
extern int g0;
extern int arr1[4];
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tbl2=%d\\n", tbl2[1](-3));
printf("tbl3=%d\\n", tbl3[1](8));
printf("f4=%d\\n", f4(-13));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
