#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int arr2[4];
extern int f3(int);
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tent1=%d\\n", init_tent1());
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("f3=%d\\n", f3(-25));
printf("mid4=%d\\n", mid4(16));
printf("tbl5=%d\\n", tbl5[1](-9));

    return 0;
}
