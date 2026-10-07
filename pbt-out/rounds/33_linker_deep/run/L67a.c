#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int f2(int);
extern int g3;
extern int arr4[4];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-43));
printf("f2=%d\\n", f2(-28));
printf("g3=%d\\n", g3);
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("tbl5=%d\\n", tbl5[1](-9));

    return 0;
}
