#include <stdio.h>
extern int g0;
extern int g1;
extern int arr2[4];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int g4;
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("tbl3=%d\\n", tbl3[1](14));
printf("g4=%d\\n", g4);
printf("tbl5=%d\\n", tbl5[1](12));

    return 0;
}
