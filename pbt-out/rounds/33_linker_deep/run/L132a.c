#include <stdio.h>
extern int g0;
extern int g1;
extern int g2;
extern int g3;
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("g2=%d\\n", g2);
printf("g3=%d\\n", g3);
printf("tbl4=%d\\n", tbl4[1](2));
printf("mid5=%d\\n", mid5(-12));

    return 0;
}
