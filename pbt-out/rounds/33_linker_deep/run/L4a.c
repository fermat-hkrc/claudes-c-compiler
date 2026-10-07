#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int mid2(int);
extern int mid3(int);
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(9));
printf("mid2=%d\\n", mid2(-19));
printf("mid3=%d\\n", mid3(-39));
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](-13));

    return 0;
}
