#include <stdio.h>
extern int g0;
extern int g1;
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int g4;
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tent2=%d\\n", init_tent2());
printf("tbl3=%d\\n", tbl3[1](4));
printf("g4=%d\\n", g4);
printf("g5=%d\\n", g5);

    return 0;
}
