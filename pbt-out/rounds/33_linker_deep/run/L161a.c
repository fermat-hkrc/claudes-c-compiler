#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int mid2(int);
extern int mid3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tent2=%d\\n", init_tent2());
printf("mid3=%d\\n", mid3(-29));
printf("tbl4=%d\\n", tbl4[1](14));
printf("f5=%d\\n", f5(-44));

    return 0;
}
