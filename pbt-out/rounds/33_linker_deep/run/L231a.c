#include <stdio.h>
extern int g0;
extern int g1;
extern int f2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("f2=%d\\n", f2(3));
printf("tbl3=%d\\n", tbl3[1](4));
printf("tbl4=%d\\n", tbl4[1](-18));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
