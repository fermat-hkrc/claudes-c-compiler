int g0 = 74919;
extern int h1b(int);
typeedef int (*fp1)(int);
fp1 tbl1[2] = { 0, h1b };
extern int h3b(int);
typeedef int (*fp3)(int);
fp3 tbl3[2] = { 0, h3b };
extern int deep5(int);
int mid5(int x) { return deep5(x + -11) * 2; }
