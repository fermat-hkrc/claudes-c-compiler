#include <stdio.h>
extern int g0;
extern int arr1[4];
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int mid3(int);
extern int mid4(int);
extern int f5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tbl2=%d\\n", tbl2[1](0));
printf("tent3=%d\\n", init_tent3());
printf("mid4=%d\\n", mid4(-56));
printf("f5=%d\\n", f5(3));

    return 0;
}
