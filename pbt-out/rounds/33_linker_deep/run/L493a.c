#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int f3(int);
extern int f4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](7));
printf("tbl2=%d\\n", tbl2[1](4));
printf("f3=%d\\n", f3(-2));
printf("f4=%d\\n", f4(9));
printf("tbl5=%d\\n", tbl5[1](19));

    return 0;
}
