#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int g3;
extern int mid4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-17));
printf("mid2=%d\\n", mid2(-16));
printf("g3=%d\\n", g3);
printf("mid4=%d\\n", mid4(6));
printf("f5=%d\\n", f5(-31));

    return 0;
}
