#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int mid3(int);
extern int f4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](1));
printf("mid2=%d\\n", mid2(-5));
printf("tent3=%d\\n", init_tent3());
printf("f4=%d\\n", f4(-27));
printf("mid5=%d\\n", mid5(16));

    return 0;
}
