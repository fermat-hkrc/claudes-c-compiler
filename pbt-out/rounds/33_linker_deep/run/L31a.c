#include <stdio.h>
extern int g0;
extern int mid1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int f3(int);
extern int mid4(int);
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(10));
printf("tbl2=%d\\n", tbl2[1](4));
printf("f3=%d\\n", f3(-10));
printf("mid4=%d\\n", mid4(-9));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
