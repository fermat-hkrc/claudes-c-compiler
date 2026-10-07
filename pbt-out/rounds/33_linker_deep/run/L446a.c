#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int f2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int g4;
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("f2=%d\\n", f2(2));
printf("tbl3=%d\\n", tbl3[1](19));
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(-19));

    return 0;
}
