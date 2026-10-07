#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int f3(int);
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("tbl2=%d\\n", tbl2[1](8));
printf("f3=%d\\n", f3(-33));
printf("mid4=%d\\n", mid4(-16));
printf("mid5=%d\\n", mid5(12));

    return 0;
}
