#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int g2;
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("g2=%d\\n", g2);
printf("tent3=%d\\n", init_tent3());
printf("tbl4=%d\\n", tbl4[1](4));
printf("f5=%d\\n", f5(-18));

    return 0;
}
