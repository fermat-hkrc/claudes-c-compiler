#include <stdio.h>
#include <stdlib.h>
static int order[10];
static int idx = 0;
static void dtor3(void) { order[idx++] = 3; }
static void dtor2(void) { order[idx++] = 2; }
static void dtor1(void) { order[idx++] = 1; }
__attribute__((constructor)) static void ctor(void) { order[idx++] = 0; atexit(dtor1); atexit(dtor2); atexit(dtor3); }
int main(void) {
    printf("main");
    return 0;
}
// After exit: dtors run in reverse registration order: 3, 2, 1
// But we can't easily check the output after exit
