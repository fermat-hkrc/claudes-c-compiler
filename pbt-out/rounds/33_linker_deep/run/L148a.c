#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int g3;
extern int g4;
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(3));
printf("tbl2=%d\\n", tbl2[1](-17));
printf("g3=%d\\n", g3);
printf("g4=%d\\n", g4);
printf("f5=%d\\n", f5(-33));

    return 0;
}
