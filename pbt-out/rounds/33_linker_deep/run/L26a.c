#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int g2;
extern int mid3(int);
extern int mid4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](13));
printf("g2=%d\\n", g2);
printf("tent3=%d\\n", init_tent3());
printf("mid4=%d\\n", mid4(-31));
printf("f5=%d\\n", f5(-32));

    return 0;
}
