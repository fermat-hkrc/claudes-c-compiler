#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int f2(int);
extern int f3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("f2=%d\\n", f2(-15));
printf("f3=%d\\n", f3(4));
printf("tbl4=%d\\n", tbl4[1](3));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
