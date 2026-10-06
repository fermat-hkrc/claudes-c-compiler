int counter(void) { static int n = 0; return ++n; }
volatile int flag = 0;
int main(void) { counter(); counter(); flag = 5; printf("%d %d\n", counter(), flag); return 0; }
