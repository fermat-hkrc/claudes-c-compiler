#include <stdio.h>
extern int g0;
extern int g1;
extern int mid2(int);
extern int f3(int);
extern int arr4[4];
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("mid2=%d\\n", mid2(-7));
printf("f3=%d\\n", f3(-9));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("tbl5=%d\\n", tbl5[1](-1));

    return 0;
}
