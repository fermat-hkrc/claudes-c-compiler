#include <stdio.h>
extern int sym0;
extern int sym1;
int main(void){
    printf("%d ", sym0);
    printf("%d ", sym1);
    putchar('\n'); return 0;
}
