#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int g2;
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](4));
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](10));

    return 0;
}
