#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int f4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-21));
printf("tent2=%d\\n", init_tent2());
printf("tbl3=%d\\n", tbl3[1](18));
printf("f4=%d\\n", f4(-41));
printf("mid5=%d\\n", mid5(-2));

    return 0;
}
