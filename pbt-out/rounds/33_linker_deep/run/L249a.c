#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int arr2[4];
extern int arr3[4];
extern int mid4(int);
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](11));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("mid4=%d\\n", mid4(-3));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
