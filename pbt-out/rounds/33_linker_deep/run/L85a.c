#include <stdio.h>
extern int g0;
extern int g1;
extern int arr2[4];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int arr4[4];
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("tbl3=%d\\n", tbl3[1](-2));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
