#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
extern int mid3(int);
extern int arr4[4];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-6));
printf("tent2=%d\\n", init_tent2());
printf("mid3=%d\\n", mid3(-51));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("tbl5=%d\\n", tbl5[1](18));

    return 0;
}
