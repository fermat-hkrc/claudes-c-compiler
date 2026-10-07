#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int g2;
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](-10));
printf("tent4=%d\\n", init_tent4());
printf("f5=%d\\n", f5(-22));

    return 0;
}
