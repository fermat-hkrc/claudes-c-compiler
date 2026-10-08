#include <stdio.h>
extern int g0;
extern int mid1(int);
extern int g2;
extern int f3(int);
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("mid1=%d\\n", mid1(-2));
printf("g2=%d\\n", g2);
printf("f3=%d\\n", f3(-38));
printf("mid4=%d\\n", mid4(-19));
printf("tbl5=%d\\n", tbl5[1](15));

    return 0;
}
