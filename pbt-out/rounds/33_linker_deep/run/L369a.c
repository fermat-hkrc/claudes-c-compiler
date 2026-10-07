#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int g2;
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-56));
printf("g2=%d\\n", g2);
printf("tent3=%d\\n", init_tent3());
printf("tbl4=%d\\n", tbl4[1](-14));
printf("mid5=%d\\n", mid5(-57));

    return 0;
}
