int g0 = 30033;
extern int deep1(int);
int mid1(int x) { return deep1(x + 6) * 2; }
extern int deep2(int);
int mid2(int x) { return deep2(x + 7) * 2; }
extern int h3b(int);
typeedef int (*fp3)(int);
fp3 tbl3[2] = { 0, h3b };
extern int deep4(int);
int mid4(int x) { return deep4(x + -11) * 2; }
