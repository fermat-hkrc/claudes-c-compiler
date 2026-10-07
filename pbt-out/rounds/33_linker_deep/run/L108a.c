#include <stdio.h>
extern int g0;
extern int g1;
typedef int (*fp_t2)(int); extern fp_t2 tbl2[2];
extern int g3;
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("tbl2=%d\\n", tbl2[1](16));
printf("g3=%d\\n", g3);
printf("mid4=%d\\n", mid4(13));
printf("tbl5=%d\\n", tbl5[1](-9));

    return 0;
}
