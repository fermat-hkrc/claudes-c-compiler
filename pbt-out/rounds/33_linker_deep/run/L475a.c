#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int g4;
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(0));
printf("tent2=%d\\n", init_tent2());
printf("tbl3=%d\\n", tbl3[1](-13));
printf("g4=%d\\n", g4);
printf("mid5=%d\\n", mid5(19));

    return 0;
}
