#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int g2;
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int g4;
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](-12));
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(-48));

    return 0;
}
