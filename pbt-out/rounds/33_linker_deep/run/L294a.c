#include <stdio.h>
extern int g0;
extern int g1;
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int f3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tbl2=%d\\n", tbl2[1](-4));
printf("f3=%d\\n", f3(0));
printf("tbl4=%d\\n", tbl4[1](7));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
