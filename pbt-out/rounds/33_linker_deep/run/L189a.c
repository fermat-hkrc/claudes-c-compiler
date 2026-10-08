#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int arr2[4];
extern int arr3[4];
extern int g4;
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("g4=%d\\n", g4);
printf("tbl5=%d\\n", tbl5[1](12));

    return 0;
}
