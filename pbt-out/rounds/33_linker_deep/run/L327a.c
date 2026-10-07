#include <stdio.h>
extern int g0;
extern int arr1[4];
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int arr3[4];
extern int arr4[4];
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tbl2=%d\\n", tbl2[1](-12));
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
