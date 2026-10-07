int g0 = -42822;
extern int deep1(int);
int mid1(int x) { return deep1(x + 4) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 7) * 2; }
extern int deep4(int);
int mid4(int x) { return deep4(x + -6) * 2; }
