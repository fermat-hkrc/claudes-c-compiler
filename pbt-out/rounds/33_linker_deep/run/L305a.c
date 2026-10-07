#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int mid2(int);
extern int f3(int);
extern int arr4[4];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-33));
printf("tent2=%d\\n", init_tent2());
printf("f3=%d\\n", f3(-30));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("tbl5=%d\\n", tbl5[1](18));

    return 0;
}
