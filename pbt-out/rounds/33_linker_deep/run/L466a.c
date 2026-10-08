#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int mid2(int);
extern int g3;
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tent2=%d\\n", init_tent2());
printf("g3=%d\\n", g3);
printf("mid4=%d\\n", mid4(6));
printf("tbl5=%d\\n", tbl5[1](6));

    return 0;
}
