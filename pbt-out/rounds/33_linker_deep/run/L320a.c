#include <stdio.h>
extern int g0;
extern int f1(int);
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int f3(int);
extern int arr4[4];
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(4));
printf("tbl2=%d\\n", tbl2[1](10));
printf("f3=%d\\n", f3(6));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("g5=%d\\n", g5);

    return 0;
}
