#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-12));
printf("mid2=%d\\n", mid2(-16));
printf("mid3=%d\\n", mid3(-52));
printf("tbl4=%d\\n", tbl4[1](-6));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
