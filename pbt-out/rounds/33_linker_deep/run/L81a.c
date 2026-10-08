#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int mid2(int);
extern int mid3(int);
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("mid2=%d\\n", mid2(16));
printf("mid3=%d\\n", mid3(-42));
printf("tent4=%d\\n", init_tent4());
printf("tbl5=%d\\n", tbl5[1](3));

    return 0;
}
