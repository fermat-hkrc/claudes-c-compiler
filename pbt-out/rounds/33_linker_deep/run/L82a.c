#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int f2(int);
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("f2=%d\\n", f2(-35));
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](-16));
printf("tbl5=%d\\n", tbl5[1](0));

    return 0;
}
