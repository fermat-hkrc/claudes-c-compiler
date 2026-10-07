#include <stdio.h>
extern int g0;
extern int g1;
extern int mid2(int);
extern int mid3(int);
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tent2=%d\\n", init_tent2());
printf("tent3=%d\\n", init_tent3());
printf("f4=%d\\n", f4(3));
printf("tbl5=%d\\n", tbl5[1](-5));

    return 0;
}
