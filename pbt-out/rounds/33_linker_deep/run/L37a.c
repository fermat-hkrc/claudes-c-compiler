#include <stdio.h>
extern int g0;
extern int g1;
extern int f2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int g4;
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("f2=%d\\n", f2(-33));
printf("tbl3=%d\\n", tbl3[1](4));
printf("g4=%d\\n", g4);
printf("tent5=%d\\n", init_tent5());

    return 0;
}
