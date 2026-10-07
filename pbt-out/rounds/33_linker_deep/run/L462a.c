#include <stdio.h>
extern int g0;
extern int g1;
extern int arr2[4];
extern int arr3[4];
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("f4=%d\\n", f4(-9));
printf("tbl5=%d\\n", tbl5[1](19));

    return 0;
}
