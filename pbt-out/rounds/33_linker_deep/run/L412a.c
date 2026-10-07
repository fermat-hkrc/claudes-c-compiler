#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int g2;
extern int mid3(int);
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(-38));
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](0));

    return 0;
}
