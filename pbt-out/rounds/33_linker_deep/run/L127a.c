#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int f2(int);
extern int mid3(int);
extern int g4;
extern int arr5[4];
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-6));
printf("f2=%d\\n", f2(-21));
printf("mid3=%d\\n", mid3(-18));
printf("g4=%d\\n", g4);
printf("arr5=%d %d\\n", arr5[0], arr5[3]);

    return 0;
}
