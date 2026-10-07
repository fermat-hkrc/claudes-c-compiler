#include <stdio.h>
extern int g0;
extern int arr1[4];
extern int arr2[4];
extern int f3(int);
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("arr1=%d %d\\n", arr1[0], arr1[3]);
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("f3=%d\\n", f3(-20));
printf("mid4=%d\\n", mid4(-50));
printf("tbl5=%d\\n", tbl5[1](0));

    return 0;
}
