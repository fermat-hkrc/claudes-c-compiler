#include <stdio.h>
extern int g0;
typedef int (*fp_t1)(int); extern fp_t1 tbl1[2];
extern int g2;
extern int g3;
extern int g4;
extern int g5;
int main(void) {
    printf("g0=%d\\n", g0);
printf("tbl1=%d\\n", tbl1[1](-1));
printf("g2=%d\\n", g2);
printf("g3=%d\\n", g3);
printf("g4=%d\\n", g4);
printf("g5=%d\\n", g5);

    return 0;
}
