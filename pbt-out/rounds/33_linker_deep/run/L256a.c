#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
extern int mid3(int);
extern int g4;
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-3));
printf("tent2=%d\\n", init_tent2());
printf("mid3=%d\\n", mid3(-16));
printf("g4=%d\\n", g4);
printf("tbl5=%d\\n", tbl5[1](-1));

    return 0;
}
