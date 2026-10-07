#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int g2;
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-20));
printf("g2=%d\\n", g2);
printf("tbl3=%d\\n", tbl3[1](0));
printf("tent4=%d\\n", init_tent4());
printf("mid5=%d\\n", mid5(-36));

    return 0;
}
