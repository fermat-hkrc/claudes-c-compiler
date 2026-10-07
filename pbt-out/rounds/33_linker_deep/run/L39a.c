#include <stdio.h>
extern int g0;
extern int g1;
extern int g2;
extern int arr3[4];
extern int mid4(int);
typedef int (*fp_t5)(int); extern fp_t5 tbl5[2];
int main(void) {
    printf("g0=%d\\n", g0);
printf("g1=%d\\n", g1);
printf("g2=%d\\n", g2);
printf("arr3=%d %d\\n", arr3[0], arr3[3]);
printf("mid4=%d\\n", mid4(-17));
printf("tbl5=%d\\n", tbl5[1](7));

    return 0;
}
