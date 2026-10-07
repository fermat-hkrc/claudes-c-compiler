#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int g2;
extern int g3;
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](19));
printf("g2=%d\\n", g2);
printf("g3=%d\\n", g3);
printf("tent4=%d\\n", init_tent4());
printf("tent5=%d\\n", init_tent5());

    return 0;
}
