#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int mid2(int);
extern int arr3[4];
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tent2=%d\\n", init_tent2());
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("f4=%d\\n", f4(-1));
printf("tbl5=%d\\n", tbl5[1](-18));

    return 0;
}
