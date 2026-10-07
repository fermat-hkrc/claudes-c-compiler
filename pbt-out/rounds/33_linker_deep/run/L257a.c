#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int f2(int);
extern int g3;
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("f2=%d\\n", f2(-3));
printf("g3=%d\\n", g3);
printf("f4=%d\\n", f4(-28));
printf("tbl5=%d\\n", tbl5[1](0));

    return 0;
}
