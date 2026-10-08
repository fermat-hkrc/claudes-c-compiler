int g0 = 95541;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
extern int deep4(int);
int mid4(int x) { return deep4(x + 11) * 2; }
extern int deep5(int);
int mid5(int x) { return deep5(x + -3) * 2; }
