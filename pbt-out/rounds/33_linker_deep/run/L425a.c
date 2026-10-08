#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int f3(int);
extern int mid4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-16));
printf("tent2=%d\\n", init_tent2());
printf("f3=%d\\n", f3(-12));
printf("tent4=%d\\n", init_tent4());
printf("f5=%d\\n", f5(-4));

    return 0;
}
