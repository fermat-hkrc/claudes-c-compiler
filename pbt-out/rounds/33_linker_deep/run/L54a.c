#include <stdio.h>
extern int g0;
extern int g1;
extern int mid2(int);
extern int f3(int);
typedef int (*fp_t4)(int); extern fp_t4 tbl4[2];
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("mid2=%d\\n", mid2(-20));
printf("f3=%d\\n", f3(-32));
printf("tbl4=%d\\n", tbl4[1](10));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
