#include <stdio.h>
extern int g0;
extern int g1;
extern int f2(int);
extern int f3(int);
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("f2=%d\\n", f2(-18));
printf("f3=%d\\n", f3(-23));
printf("f4=%d\\n", f4(-25));
printf("tbl5=%d\\n", tbl5[1](-19));

    return 0;
}
