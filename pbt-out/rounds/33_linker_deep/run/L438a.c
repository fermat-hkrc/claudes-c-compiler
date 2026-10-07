#include <stdio.h>
extern int g0;
extern int f1(int);
extern int arr2[4];
typedef int (*fp_t3)(int); extern fp_t3 tbl3[2];
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(-22));
printf("arr2=%d %d\\n", arr2[0], arr2[3]);
printf("tbl3=%d\\n", tbl3[1](10));
printf("mid4=%d\\n", mid4(-34));
printf("mid5=%d\\n", mid5(-48));

    return 0;
}
