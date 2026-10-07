#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int f3(int);
extern int f4(int);
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](9));
printf("tent2=%d\\n", init_tent2());
printf("f3=%d\\n", f3(8));
printf("f4=%d\\n", f4(-25));
printf("g5=%d\\n", g5);

    return 0;
}
