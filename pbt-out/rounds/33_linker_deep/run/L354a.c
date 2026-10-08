#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int g4;
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("mid2=%d\\n", mid2(-26));
printf("tbl3=%d\\n", tbl3[1](-18));
printf("g4=%d\\n", g4);
printf("f5=%d\\n", f5(12));

    return 0;
}
