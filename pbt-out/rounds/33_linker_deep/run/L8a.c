#include <stdio.h>
extern int g0;
extern int f1(int);
extern int mid2(int);
extern int f3(int);
extern int g4;
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("f1=%d\\n", f1(1));
printf("mid2=%d\\n", mid2(-1));
printf("f3=%d\\n", f3(-32));
printf("g4=%d\\n", g4);
printf("tbl5=%d\\n", tbl5[1](14));

    return 0;
}
