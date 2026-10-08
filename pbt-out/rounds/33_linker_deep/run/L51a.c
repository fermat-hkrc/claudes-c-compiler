#include <stdio.h>
extern int g0;
extern int g1;
extern int mid2(int);
extern int mid3(int);
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("mid2=%d\\n", mid2(-58));
printf("mid3=%d\\n", mid3(-54));
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](-17));

    return 0;
}
