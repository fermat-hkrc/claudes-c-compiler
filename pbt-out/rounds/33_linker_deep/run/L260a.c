#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int g3;
extern int f4(int);
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-1));
printf("mid2=%d\\n", mid2(17));
printf("g3=%d\\n", g3);
printf("f4=%d\\n", f4(-2));
printf("g5=%d\\n", g5);

    return 0;
}
