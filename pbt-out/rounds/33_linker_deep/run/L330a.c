#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int mid3(int);
extern int g4;
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](19));
printf("mid2=%d\\n", mid2(-38));
printf("mid3=%d\\n", mid3(-28));
printf("g4=%d\\n", g4);
printf("f5=%d\\n", f5(-26));

    return 0;
}
