#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int mid2(int);
extern int mid3(int);
extern int arr4[4];
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](5));
printf("mid2=%d\\n", mid2(-50));
printf("mid3=%d\\n", mid3(-22));
printf("arr4=%d %d\\n", arr4[0], arr4[3]);
printf("mid5=%d\\n", mid5(-55));

    return 0;
}
