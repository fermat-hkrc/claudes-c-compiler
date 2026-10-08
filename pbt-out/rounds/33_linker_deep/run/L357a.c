#include <stdio.h>
extern int g0;
extern int g1;
extern int f2(int);
extern int mid3(int);
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("f2=%d\\n", f2(9));
printf("mid3=%d\\n", mid3(-55));
printf("f4=%d\\n", f4(-38));
printf("tbl5=%d\\n", tbl5[1](13));

    return 0;
}
