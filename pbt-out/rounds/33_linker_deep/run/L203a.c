#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int g2;
extern int g3;
extern int mid4(int);
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](1));
printf("g2=%d\\n", g2);
printf("g3=%d\\n", g3);
printf("mid4=%d\\n", mid4(-59));
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
