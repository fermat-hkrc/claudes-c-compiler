#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int g3;
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-3));
printf("mid2=%d\\n", mid2(-28));
printf("g3=%d\\n", g3);
printf("tent4=%d\\n", init_tent4());
printf("mid5=%d\\n", mid5(-25));

    return 0;
}
