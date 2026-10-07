#include <stdio.h>
extern int call_a(void);
extern int call_b(void);
int main(void) { printf("%d %d\n", call_a(), call_b()); return 0; }
