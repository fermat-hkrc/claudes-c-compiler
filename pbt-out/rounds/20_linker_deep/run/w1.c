__attribute__((weak)) int wfunc(void) { return 1; }
extern int wfunc(void);
int main(void) { printf("%d\n", wfunc()); return 0; }
