#include <stdio.h>
extern int g0;
extern int f1(int);
extern int arr2[4];
extern int arr3[4];
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-13));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("tbl4=%d\\n", tbl4[1](-5));
printf("f5=%d\\n", f5(-41));

    return 0;
}
