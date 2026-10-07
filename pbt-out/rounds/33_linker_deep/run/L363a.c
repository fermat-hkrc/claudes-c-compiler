#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int g2;
extern int mid3(int);
extern int mid4(int);
extern int mid5(int);
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](4));
printf("g2=%d\\n", g2);
printf("mid3=%d\\n", mid3(-39));
printf("mid4=%d\\n", mid4(5));
printf("mid5=%d\\n", mid5(2));

    return 0;
}
