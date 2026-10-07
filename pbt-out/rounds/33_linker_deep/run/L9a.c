#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int f2(int);
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-34));
printf("f2=%d\\n", f2(-23));
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](-4));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
