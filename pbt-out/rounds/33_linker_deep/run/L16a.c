#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int mid2(int);
extern int f3(int);
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tent2=%d\\n", init_tent2());
printf("f3=%d\\n", f3(-1));
printf("f4=%d\\n", f4(-18));
printf("tbl5=%d\\n", tbl5[1](-15));

    return 0;
}
