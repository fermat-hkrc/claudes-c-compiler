#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int f3(int);
extern int g4;
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](14));
printf("tent2=%d\\n", init_tent2());
printf("f3=%d\\n", f3(-2));
printf("g4=%d\\n", g4);
printf("tbl5=%d\\n", tbl5[1](-18));

    return 0;
}
