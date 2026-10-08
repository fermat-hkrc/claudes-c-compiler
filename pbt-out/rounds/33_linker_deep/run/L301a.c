#include <stdio.h>
extern int g0;
extern int arr1[4];
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int g3;
extern int f4(int);
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("tbl2=%d\\n", tbl2[1](13));
printf("g3=%d\\n", g3);
printf("f4=%d\\n", f4(-17));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
