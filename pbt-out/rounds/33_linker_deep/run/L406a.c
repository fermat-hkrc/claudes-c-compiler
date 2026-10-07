#include <stdio.h>
extern int g0;
extern int g1;
extern int f2(int);
extern int mid3(int);
extern int g4;
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("f2=%d\\n", f2(-34));
printf("mid3=%d\\n", mid3(-16));
printf("g4=%d\\n", g4);
printf("tbl5=%d\\n", tbl5[1](-5));

    return 0;
}
