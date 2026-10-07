#include <stdio.h>
extern int g0;
extern int g1;
extern int mid2(int);
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tent2=%d\\n", init_tent2());
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](18));
printf("tent5=%d\\n", init_tent5());

    return 0;
}
