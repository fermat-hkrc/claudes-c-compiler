#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int g2;
extern int mid3(int);
extern int g4;
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(-34));
printf("g4=%d\\n", g4);
printf("tbl5=%d\\n", tbl5[1](18));

    return 0;
}
