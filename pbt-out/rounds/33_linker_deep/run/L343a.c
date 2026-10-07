#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int mid2(int);
extern int arr3[4];
extern int arr4[4];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-50));
printf("mid2=%d\\n", mid2(-59));
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("tbl5=%d\\n", tbl5[1](17));

    return 0;
}
